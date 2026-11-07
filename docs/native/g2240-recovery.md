# Game 2240 (simon the sorcerer (1993)(adventure soft)[cr pdx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02240). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (8113728 top-level bytes in 470 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 19 of 19 units identical (bytes and relocations), 21 raw-word lines |
| Functions named and described | pass, 810 of 810 named (0 author symbols), 810 of 810 described (81 by rule, 722 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3660 of 3661 (99.97%) |
| Recovered without load address (counted in proof 1, flagged) | 38707 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 18377 of 19322 instructions identical in start and length, 2 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 4741274 |
| code | 63930 |
| data | 2700768 |
| padding | 112140 |
| unused | 495616 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 1 of 9).adf:run | 2 | 117 | 2 |
| Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 1 of 9).adf:pdxintro | 2 | 267 | 8 |
| Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 1 of 9).adf:runback | 3 | 709 | 39 |
| disk-validator | 1 | 386 | 1 |
| Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 1 of 9).adf:endcli | 2 | 117 | 2 |
| Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 1 of 9).adf:runit | 13 | 16498 | 694 |
| Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 1 of 9).adf:iconx | 12 | 806 | 37 |
| Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 1 of 9).adf:bootblock | 1 | 10 | 1 |
| Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 2 of 9).adf:bootblock | 1 | 10 | 1 |
| Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 3 of 9).adf:bootblock | 1 | 10 | 1 |
| Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 4 of 9).adf:bootblock | 1 | 10 | 1 |
| Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 5 of 9).adf:bootblock | 1 | 10 | 1 |
| Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 6 of 9).adf:bootblock | 1 | 10 | 1 |
| Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 7 of 9).adf:bootblock | 1 | 10 | 1 |
| Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 9 of 9).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 322 | 18 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 1 of 9).adf:gameamiga (raw code file, load address unknown) | 1 | 0 | 0 |
| Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 5 of 9).adf:0141.pkd (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 1 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 1 of 9).adf:7tune: high-entropy data file (7.85 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 1 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 1 of 9).adf:1552.pkd: high-entropy data file (7.72 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 2 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 2 of 9).adf:14tune: high-entropy data file (7.86 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 2 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 2 of 9).adf:7tune: high-entropy data file (7.85 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 2 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 2 of 9).adf:0312.pkd: high-entropy data file (7.65 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 2 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 2 of 9).adf:15tune: high-entropy data file (7.79 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 2 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 2 of 9).adf:0362.pkd: high-entropy data file (7.70 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 2 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 2 of 9).adf:8tune: high-entropy data file (7.76 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 2 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 2 of 9).adf:0332.pkd: high-entropy data file (7.61 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 2 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 2 of 9).adf:0322.pkd: high-entropy data file (7.65 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 2 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 2 of 9).adf:16tune: high-entropy data file (7.71 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 2 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 2 of 9).adf:0302.pkd: high-entropy data file (7.65 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 3 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 3 of 9).adf:0452.pkd: high-entropy data file (7.62 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 3 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 3 of 9).adf:0102.pkd: high-entropy data file (7.64 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 3 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 3 of 9).adf:0tune: high-entropy data file (7.82 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 3 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 3 of 9).adf:12tune: high-entropy data file (7.79 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 3 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 3 of 9).adf:0782.pkd: high-entropy data file (7.62 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 3 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 3 of 9).adf:0772.pkd: high-entropy data file (7.67 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 3 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 3 of 9).adf:0432.pkd: high-entropy data file (7.70 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 3 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 3 of 9).adf:0702.pkd: high-entropy data file (7.76 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 3 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 3 of 9).adf:0752.pkd: high-entropy data file (7.67 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 3 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 3 of 9).adf:0462.pkd: high-entropy data file (7.80 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 3 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 3 of 9).adf:0742.pkd: high-entropy data file (7.63 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 3 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 3 of 9).adf:1tune: high-entropy data file (7.77 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 3 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 3 of 9).adf:22tune: high-entropy data file (7.81 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 3 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 3 of 9).adf:0792.pkd: high-entropy data file (7.72 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 3 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 3 of 9).adf:0442.pkd: high-entropy data file (7.64 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 3 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 3 of 9).adf:0712.pkd: high-entropy data file (7.72 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 3 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 3 of 9).adf:0802.pkd: high-entropy data file (7.69 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 3 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 3 of 9).adf:0482.pkd: high-entropy data file (7.77 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 3 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 3 of 9).adf:0762.pkd: high-entropy data file (7.76 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 3 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 3 of 9).adf:0562.pkd: high-entropy data file (7.67 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 3 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 3 of 9).adf:23tune: high-entropy data file (7.84 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 4 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 4 of 9).adf:0822.pkd: high-entropy data file (7.71 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 4 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 4 of 9).adf:0812.pkd: high-entropy data file (7.74 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 4 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 4 of 9).adf:0492.pkd: high-entropy data file (7.76 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 4 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 4 of 9).adf:10tune: high-entropy data file (7.71 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 4 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 4 of 9).adf:0852.pkd: high-entropy data file (7.71 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 4 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 4 of 9).adf:28tune: high-entropy data file (7.85 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 4 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 4 of 9).adf:0832.pkd: high-entropy data file (7.74 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 4 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 4 of 9).adf:32tune: high-entropy data file (7.83 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 4 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 4 of 9).adf:1tune: high-entropy data file (7.77 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 4 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 4 of 9).adf:29tune: high-entropy data file (7.80 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 4 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 4 of 9).adf:0842.pkd: high-entropy data file (7.78 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 5 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 5 of 9).adf:0882.pkd: high-entropy data file (7.74 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 5 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 5 of 9).adf:0142.pkd: high-entropy data file (7.67 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 5 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 5 of 9).adf:5tune: high-entropy data file (7.80 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 5 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 5 of 9).adf:17tune: high-entropy data file (7.75 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 5 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 5 of 9).adf:0122.pkd: high-entropy data file (7.71 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 5 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 5 of 9).adf:1tune: high-entropy data file (7.77 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 5 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 5 of 9).adf:0872.pkd: high-entropy data file (7.70 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 5 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 5 of 9).adf:4tune: high-entropy data file (7.83 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 6 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 6 of 9).adf:0892.pkd: high-entropy data file (7.63 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 6 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 6 of 9).adf:0732.pkd: high-entropy data file (7.67 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 6 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 6 of 9).adf:21tune: high-entropy data file (7.79 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 6 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 6 of 9).adf:0862.pkd: high-entropy data file (7.68 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 6 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 6 of 9).adf:26tune: high-entropy data file (7.81 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 6 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 6 of 9).adf:0122.pkd: high-entropy data file (7.71 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 6 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 6 of 9).adf:3tune: high-entropy data file (7.80 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 6 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 6 of 9).adf:1tune: high-entropy data file (7.77 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 6 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 6 of 9).adf:0722.pkd: high-entropy data file (7.78 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 6 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 6 of 9).adf:0132.pkd: high-entropy data file (7.61 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 6 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 6 of 9).adf:27tune: high-entropy data file (7.87 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 6 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 6 of 9).adf:2tune: high-entropy data file (7.82 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 7 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 7 of 9).adf:0972.pkd: high-entropy data file (7.63 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 7 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 7 of 9).adf:24tune: high-entropy data file (7.83 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 7 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 7 of 9).adf:0982.pkd: high-entropy data file (7.61 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 7 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 7 of 9).adf:20tune: high-entropy data file (7.80 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 7 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 7 of 9).adf:6tune: high-entropy data file (7.79 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 7 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 7 of 9).adf:0412.pkd: high-entropy data file (7.70 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 7 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 7 of 9).adf:25tune: high-entropy data file (7.75 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 7 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 7 of 9).adf:4tune: high-entropy data file (7.83 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 7 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 7 of 9).adf:1062.pkd: high-entropy data file (7.70 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 8 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 8 of 9).adf:19tune: high-entropy data file (7.73 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 8 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 8 of 9).adf:30tune: high-entropy data file (7.84 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 8 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 8 of 9).adf:18tune: high-entropy data file (7.60 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 9 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 9 of 9).adf:31tune: high-entropy data file (7.76 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 9 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 9 of 9).adf:7tune: high-entropy data file (7.85 bits/byte), classified as packed data
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 9 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 9 of 9).adf:33tune: high-entropy data file (7.77 bits/byte), classified as packed data
- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 00C0) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address: 278 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 1 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 1 of 9).adf:gameamiga: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 5 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 5 of 9).adf:0141.pkd: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- simon the sorcerer (1993)(adventure soft)[cr pdx](disk 8 of 9)/Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 8 of 9).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- 2 never-loaded region(s), 38707 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 1 of 9).adf:pdxintro

## Copy protection found

- none found

## Functions


### Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 1 of 9).adf:run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of one register to another and then performs a short relative branch.

### Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 1 of 9).adf:pdxintro

- `parse_amigados_hunks`: Parses a data stream containing AmigaDOS hunk type codes to allocate memory and relocate data.
- `decrement_and_loop`: Decrements a word value in a register until it reaches zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `process_bit_stream_2`: Performs bitwise shifts and logical operations on data in registers and memory while manipulating a buffer.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `process_bit_stream`: Performs bitwise shifts and logical operations on data in registers and memory.
- `execute_trap_sequence`: Moves a word from a table into a register, performs a logical operation, executes a software interrupt, and performs a logical OR on a register.

### Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 1 of 9).adf:runback

- `call_subroutine`: Performs a single subroutine call.
- `run_background_task`: Parses a command line for an optional load delay and a program name with parameters, then starts the named program as a background task, handling errors such as missing command, missing file, directory instead of file, out of memory and usage errors, and prints status messages.
- `clear_table_and_recurse`: Clears a block of forty words starting at an offset from a base pointer, stores a local stack address and a global variable into subsequent fields, and calls another routine.
- `call_optional_handler_and_recurse`: Checks if a global variable is non-zero to call a function pointer, then calls itself recursively.
- `process_dialogue_text`: Parses a string to extract individual words and updates global state fields.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `run_back_loop_handler`: Performs a short loop calling another routine ten times and then inspects several global variables.
- `load_short_constant`: Loads the value 32767 into a data register and branches to another location.
- `append_string_with_count`: Walks to the end of the destination string and appends bytes from the source, stopping at the source terminator or when the count argument runs out, then null-terminates the destination and returns it.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `copy_bytes_zero_padded`: Copies up to a counted number of bytes from the source, stopping early after a zero byte, then fills the rest of the destination up to that count with zeros.
- `validate_and_clear_table_entry`: Checks if an index is within a specific range and if a table entry is non-zero, then calls another routine and clears the entry if valid.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_offset_102`: Calls a function at a specific offset within a library retrieved from a global pointer.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `execute_indirect_function_pointer`: Retrieves a function pointer from a nested structure within a global table and executes it.
- `execute_indirect_function_call`: Fetches a pointer from a location offset from a global variable and executes the function found at a secondary offset within that pointer's target.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `execute_indirect_function_call_2`: Loads a pointer from a location offset from a global variable and executes the function at that address.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_2`: Retrieves a function pointer from a nested structure within a global table and executes it.
- `execute_indirect_call`: Performs an indirect function call by traversing multiple pointers stored at an offset from a value in a register.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_function`: Calls a function from an Amiga library using an offset derived from a global base address.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_function_2`: Executes a library routine by accessing a function pointer via a global library base.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### disk-validator

- `format_error_string`: Converts a numeric value into a character string and stores it in a buffer.

### Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 1 of 9).adf:endcli

- `reallocate_and_copy_buffer`: Finds the maximum value in a linked list and uses it to resize a buffer by writing a header and filling it with incrementing values.
- `move_and_branch_2`: Moves a long value from one register to another and then performs a relative branch.

### Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 1 of 9).adf:runit

- `initialize_engine_environment`: Clears a memory block and sets up internal stack and pointer structures.
- `load_stack_to_register`: Loads a long value from a location on the stack into a register.
- `execute_callback_chain`: Executes a series of function pointers stored in a global table and memory structures, while managing a stack-like value through a pointer.
- `initialize_system_environment`: Sets global values, performs recursive calls, initializes graphics and data structures, manages memory blocks via linked lists, and executes various callback routines based on structure flags.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `call_load_rgb4`: Calls a routine to load a four-bit RGB graphics palette.
- `store_passed_parameters_2`: Copies seven passed parameters into a series of global variables.
- `execute_indirect_function`: Loads a function pointer from a global variable and executes it.
- `call_close_workbench`: Calls a routine to close the workbench.
- `call_indirect_via_offset_2`: Calls a function located at an address calculated by adding a negative offset to a global variable.
- `get_table_word`: Returns a sixteen-bit value from a table using an index.
- `get_table_long`: Returns a thirty-two-bit value from a table using an index.
- `set_table_word`: Stores a sixteen-bit value into a table at a specified index.
- `get_global_pointer`: Returns a pointer to a specific global variable or a fallback global address if a certain pointer is null.
- `check_and_load_address`: Loads a longword from a memory location into a register and conditionally branches based on its value.
- `return_global_long`: Returns a long value stored in a global variable.
- `increment_global_pointer`: Loads a byte from a global variable, increments that variable by one, and returns the original byte value.
- `fetch_word_from_table`: Reads a sixteen-bit value from a global table and recursively calls itself if the value falls within a specific range.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies d7 to d0, then copies (a7)+ to d7, then returns.
- `read_two_bytes_as_word`: Reads two consecutive bytes from a global variable, increments the variable by two, and returns the value as a sixteen-bit integer.
- `read_big_endian_word_advance`: Reads two consecutive bytes from a global byte pointer, combines them as a big-endian 16-bit value (first byte as the high byte), advances the pointer by two, and returns the result as an unsigned integer.
- `fetch_word_from_global`: Reads a sixteen-bit word from a global variable and returns it.
- `compare_three_values`: Checks if a short integer at a pointer offset is zero, then compares three subsequent short integers against specific global values or special constants.
- `update_global_pointer_5`: Assigns the value of one global variable to another before calling another routine.
- `traverse_linked_list_structure`: Iterates through a linked list or data structure where each element contains an offset to the next node.
- `evaluate_command_expression`: Parses and evaluates a sequence of command tokens and operands to determine a boolean result.
- `search_linked_list_3`: Traverses a linked list to find a node containing a specific value.
- `create_linked_node`: Allocates a small memory block, initializes it with a provided value and a pointer, and adds it to a global linked list.
- `insert_into_linked_list`: Inserts a new node into a linked list by allocating memory and updating the surrounding pointers.
- `resolve_linked_list_pointers`: Retrieves and validates two pointers from a data structure, potentially following offsets, and extracts two short values from the resulting nodes.
- `resolve_global_pointer`: Retrieves a pointer from a global variable and performs several checks to determine a final address or set specific global values.
- `reset_global_values`: Sets two consecutive global variables to a value of all ones.
- `process_callback_chain_2`: An identical implementation to R1 that processes structure-based function pointers and updates global state.
- `process_linked_list_events`: Inserts a block of data into a linked list and iterates through the list, executing function pointers stored within the list elements.
- `process_text_string_buffer`: Iterates through a string of characters to calculate text wrapping and line breaks.
- `execute_wrapper_routines`: Calls three separate routines using a global variable and a provided parameter.
- `allocate_and_zero_memory`: Attempts to allocate a memory block of a specified size and, if successful, fills the resulting space with zeros.
- `initialize_allocation_state`: Calls another routine with a shifted version of a parameter and updates two global variables.
- `store_string_pointers`: Iterates through a specified number of null-terminated strings, storing each string pointer into a table and updating a global count with the total number of strings processed.
- `fetch_table_value`: Calculates an offset based on a parameter and retrieves a four-byte value from a table.
- `fetch_table_entry_4`: Retrieves a four-byte value from a table using a provided index multiplied by four, or calls itself recursively if the index is too large.
- `check_pointer_word`: Loads a word from a location pointed to by a stack offset and branches based on whether that word is non-zero.
- `remove_linked_list_node_2`: Searches a linked list for a specific node and removes it by updating the preceding node's pointer to bypass the target.
- `unlink_list_item`: Traverses a linked list to find a specific node and removes it by updating the surrounding pointers.
- `calculate_offset_address`: Adds a longword from a global variable to a word from a memory location and moves the result into an address register.
- `invoke_wrapper_with_pointer`: Passes the address of a global variable to a wrapper routine that executes a specific function.
- `test_word_and_branch`: Tests a word at a location pointed to by a register and branches if it is zero.
- `update_memory_offsets`: Calculates and stores two word-sized offset values into a destination pointed to by a register.
- `clear_memory_at_pointer`: Sets two word-sized values at a location pointed to by a register to zero.
- `initialize_list_structure`: Allocates a sixteen-byte memory block, clears its contents, and updates a global pointer and a counter.
- `write_word_to_offset`: Stores a word into a memory location at a specific offset from a provided pointer.
- `search_linked_list`: Traverses a linked list starting from an offset within a structure to find a node containing a specific word value.
- `traverse_linked_list`: Iterates through a linked list of words until a specific value is found or a boundary is reached.
- `calculate_address_sum`: Adds a long value from a pointer in a register to a word value from a global variable and stores the result in a register before branching.
- `return_zero`: Clears the accumulator and returns to the caller.
- `allocate_and_initialize_structure`: Allocates memory and populates the new block with data from a source and a provided parameter.
- `compare_address_ranges`: Saves registers to the stack and compares a pointer against a calculated address range.
- `decrement_and_branch`: Increments a long register, adds a constant to an address register, decrements a long register, and branches.
- `execute_wrapper_calls`: Invokes three different routines by passing a parameter and utilizing a global variable.
- `compare_and_branch`: Compares a value from a global variable against a memory location and branches based on the result.
- `decrement_and_branch_short`: Adds a constant to an address register, decrements a long register, and branches.
- `return_zero_2`: This routine is empty and simply returns a zero value to the caller.
- `get_global_value`: Returns a value stored in a global variable.
- `return_global_value`: Returns the value stored in a specific global variable.
- `clear_global_value`: Sets a global variable to zero.
- `search_list_with_three`: Calls another routine with a constant value of three passed as an argument.
- `search_list_with_two`: Calls another routine with a constant value of two passed as an argument.
- `search_list_with_one`: Calls another routine with a constant value of one passed as an argument.
- `search_list_with_nine`: Calls another routine with a constant value of nine passed as an argument.
- `traverse_memory_tree`: Recursively traverses a structure by checking if a value at a pointer is non-zero and then processing related memory offsets.
- `pass_stack_value_to_d0`: Moves a long value from the stack into a data register and returns.
- `call_stack_to_register_move`: Calls a routine that moves a long value from the stack into a data register.
- `update_structure_field`: Searches for a structure in a list and, if found, writes a word value into one of four possible indexed offsets within that structure.
- `clear_memory_block`: Sets four consecutive word-sized values to zero starting at an offset from a provided pointer.
- `store_long_word`: Splits a 32-bit value into four bytes and stores them in a memory buffer.
- `fetch_next_byte`: Returns a byte from a global table by incrementing a pointer, or calls another routine if a flag is not set.
- `copy_string_with_conditions`: The routine performs a series of operations involving a helper function and a conditional string copy into a destination buffer.
- `fetch_next_word`: Retrieves two consecutive bytes from a global table to form a word, incrementing the table pointer by two.
- `split_parameter_and_call`: Calls another routine twice, passing the high word and then the low word of a 16-bit parameter.
- `combine_two_words`: Calls another routine twice to retrieve two values and combines them into a single 32-bit integer.
- `call_split_parameter`: Compares a value at a specific address to a constant and, if they match, pushes a value to the stack and calls another routine.
- `execute_recursive_calls`: This routine performs two consecutive calls to itself.
- `transform_combined_value`: Retrieves two values from another routine, combines them into a 32-bit integer, and applies a mathematical transformation if the result is not negative.
- `pass_through_value`: Returns a value retrieved from another routine without modification.
- `allocate_array_of_pointers`: Allocates a block of memory to hold an array of pointers and populates each entry by calling another routine.
- `unpack_bitfield_data`: Reads data from a source and uses bitwise operations to determine how many subsequent values to extract and store into a newly allocated structure.
- `unpack_structured_data`: Retrieves multiple words from a source to populate an array and processes a variable number of subsequent values.
- `convert_token_to_numeric_value`: Iterates through a string from a table and converts specific character sequences into two-byte numeric values.
- `decrement_d5_and_branch`: Decrements a word in register d5 and branches to another location.
- `decrement_d5_and_branch_2`: Decrements a word in register d5 by three and branches to another location.
- `decrement_d5_and_branch_3`: Decrements a word in register d5 by five and branches to another location.
- `decrement_d5_and_branch_4`: Decrements a word in register d5 by seven and branches to another location.
- `parse_string_token`: Parses a string from a global table by identifying specific character sequences and converting them into numeric values.
- `decompress_string_data`: Reads a stream of bytes, expanding specific control characters into words, and copies the resulting sequence into a destination buffer.
- `decompress_and_insert`: Repeatedly retrieves data from a source and inserts it into a destination using a specific expansion routine.
- `process_linked_list_stream`: Iterates through a loop, retrieving a value from a source and using it to fetch a pointer to another value, which is then used to perform an insertion into a destination via a linked list node allocation.
- `copy_string_data_2`: Copies a sequence of bytes from a source buffer to a destination buffer based on a specified length.
- `initialize_data_structures`: Allocates memory blocks and populates them with data from a stream, including the initialization of several metadata pointers.
- `process_data_stream`: Copies a null-terminated string from a global pointer into a local buffer and then processes it through a series of nested routine calls and memory management operations.
- `parse_encoded_table`: Iterates through a sequence of bytes to calculate a total size and extract values from a table.
- `convert_digits_to_string`: The routine reads characters and converts them into numeric digit characters by adding an offset, storing the results into a buffer derived from a global variable.
- `serialize_linked_list_data`: Iterates through a linked list of structures to calculate the total element count and then writes their contents into a destination buffer.
- `process_linked_list_elements`: Iterates through a linked list of structures to perform conditional bitmask checks and data processing.
- `process_linked_structures`: Iterates through a linked list of structures to perform conditional string copies and bitmask checks on the elements.
- `iterate_and_process_data_structures`: Iterates through a linked list of structures, performing conditional searches, string copies, and bitmask checks on the elements.
- `process_complex_structure`: Iterates through a structure using a pointer, performing multiple conditional calls and updating a global variable based on the results.
- `increment_pointer_and_decrement_counter`: Increments a pointer stored in a register by a constant offset, decrements a long integer in a register, and performs a relative branch to another routine.
- `execute_iterative_callbacks_2`: Performs several loops that call various routines a specific number of times and conditionally calls another routine based on a global variable.
- `execute_iterative_callbacks`: Executes a series of loops that call various routines a specific number of times and conditionally calls another routine based on a global variable.
- `execute_iterative_conditional_callbacks`: Executes a routine sixteen times in a loop, followed by a second loop of sixteen iterations that calls another routine and conditionally executes a third routine if a global variable is non-zero.
- `execute_conditional_callback_2`: Iterates sixteen times calling a routine, then conditionally calls another routine using a specific address if a global variable is non-zero.
- `execute_conditional_callback`: Saves a global variable, calls two routines, and then calls a third routine using a specific address if the original global variable was non-zero.
- `decrement_and_loop_pointer`: Decrements a value from the stack and uses it to iterate through a table of pointers, adjusting the pointer by a fixed offset in each step.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `initialize_game_data_structures_2`: Populates various global tables and memory structures by iterating through loops to assign values, including bitmask-based property assignments and the initialization of numeric arrays.
- `initialize_game_data_structures`: Iterates through a series of loops to populate various global tables and memory structures with values retrieved from other routines.
- `check_and_call_routine`: Calls a routine, compares a value from the stack to a register, and conditionally calls another routine or branches.
- `initialize_object_attribute_tables_2`: Iterates through a list of object structures to populate their attributes, including bitmask-driven property arrays and coordinate or index data.
- `initialize_object_attribute_tables`: Iterates through a list of object structures to populate their attributes, including bitmask-driven property arrays and coordinate or index values, from a source table.
- `populate_object_attribute_tables_2`: Iterates through a list of object structures to populate their attributes by fetching words from a global table.
- `populate_object_attribute_tables`: Iterates through a collection of data structures to fetch words from a global table and store them into specific offsets of the objects.
- `adjust_pointer_and_branch`: Increments a pointer by a fixed offset, decrements a value on the stack, and branches to another routine.
- `populate_word_table_4`: Fetches words from a source and stores them into various global arrays via multiple loops.
- `populate_word_table_3`: Iterates through loops to fetch words from a global source and store them into a global array.
- `populate_word_table_2`: Iterates through a table twice to fetch words from a global source and store them into a global array.
- `populate_word_table`: Iterates sixteen times to fetch words from a global table and store them into a global array.
- `check_structure_status`: Calls two routines, clears a global variable, calls another routine, and returns the value of a global variable.
- `process_complex_data_loop`: Initializes a memory block with zeros and enters a loop that performs multiple recursive and conditional calls based on data structure traversal.
- `increment_and_compare_value`: Compares a longword in a register against a constant and performs conditional branching and arithmetic on a register based on the result.
- `call_subroutines_conditionally_2`: Compares a value in a register against a constant and, if it is less than that constant, pushes two different pointers onto the stack and calls a subroutine twice.
- `calculate_buffer_difference`: Compares two values and, if they meet a specific condition, calls another routine and returns the difference between them.
- `traverse_indexed_table`: Calculates an offset from a data structure to iterate through a table in increments of ten.
- `check_global_zero`: Tests if a global variable is zero and, if so, calls another routine.
- `fetch_table_entry_3`: Calculates an offset from a data structure to retrieve a byte from a table.
- `copy_memory_block_4`: Copies sixteen words from a global location to itself if two global conditions are met.
- `decrement_and_recurse`: Decrements a parameter and calls itself recursively if the value is not one.
- `process_character_buffer`: Checks a global flag and initializes variables before iterating through a character string to process its contents.
- `invoke_conditional_logic`: Calls a routine to process conditional logic and then calls another routine to process a conditional callback.
- `initialize_structure_block`: Sets up several global variables and initializes a new memory block with specific values.
- `process_conditional_callback`: Calls another routine with a specific constant and, if the result is non-zero, passes that result to a second routine before returning zero.
- `execute_infinite_loop_4`: Enters an infinite loop that repeatedly calls two different routines.
- `execute_conditional_sequence`: Checks two global variables and, if they match specific values, executes a series of nested routine calls with various arguments.
- `traverse_nested_structure`: Reads a word from a pointer, uses it to call another routine, and then conditionally calls a third routine using the returned value.
- `fetch_table_pointer`: Retrieves a pointer from a table using a value from the stack shifted left by two and returns it or one.
- `compare_global_and_pointer`: Retrieves a global pointer, compares a value at that pointer with a word from another location, and returns one or zero based on the result.
- `return_zero_3`: This routine is empty and simply returns a value of zero to its caller.
- `compare_sum_with_pointer`: Checks if a global pointer is null or if the sum of a global value and a value from that pointer equals a target integer.
- `compare_sum_with_register`: Adds a value from a global pointer to a word from another location and compares the sum against a register.
- `return_zero_4`: This routine is empty and simply returns a value of zero to the caller.
- `fetch_table_entry_by_index`: Calculates an address by scaling an index from the stack and adding it to a base pointer, then loads the value at that address into a register.
- `compare_register_with_offset`: Compares a longword in a register with a word stored at an offset from another location.
- `return_zero_5`: This routine is empty and returns zero to the caller.
- `update_structure_values`: Writes several constant values into a memory structure at specific offsets relative to a pointer retrieved from a global variable.
- `clear_fifth_bit_2`: Clears the fifth bit of a global variable.
- `update_parameter_table_2`: Checks a global variable and a flag before either waiting for another global variable to reach zero or storing a passed parameter into a table.
- `find_closest_coordinates_2`: Iterates through a list of tables to find the entry with the minimum calculated distance from two input parameters and stores the resulting index and table position.
- `set_global_byte`: Stores the value zero-ff into a global variable.
- `decrement_and_traverse_list`: Decrements a long value pointed to by a register and updates that register with a value from an offset of the previous pointer.
- `tiny_movea_rts`: Small routine of 2 instructions that copies (a7)+ to a5, then returns.
- `allocate_memory_block_2`: Allocates a memory block by calculating a size and inserting the new segment into a linked list of memory blocks.
- `remove_linked_list_node`: Removes a specific address from a linked list by updating the pointer of the preceding element to skip the target.
- `follow_data_offset`: If a global flag is zero, it reads a short value from an offset within a provided pointer and calls another routine with that value.
- `process_linked_list_nodes`: Iterates through a linked list, removing nodes and updating a global pointer based on calculated offset values.
- `process_list_node`: Checks a pointer within a structure and, if not null, performs a recursive call with normalized fixed-point values.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `manage_task_chain`: Performs a complex sequence of operations involving memory allocation, string copying, and linked list manipulation via multiple subroutines.
- `execute_complex_sequence_wrapper`: Sets a bit in a global variable, stores two parameters into global locations, and calls several other routines.
- `copy_string_to_buffer_3`: Copies a null-terminated string into a global buffer.
- `set_bit_and_copy_data`: Sets a bit in a global variable and, if certain conditions are met, performs a block copy of data.
- `clear_seventh_bit`: Clears the seventh bit of a global variable and calls another routine if a condition is met.
- `assign_global_pointer`: Copies the value of one global variable into another.
- `count_set_bits`: Counts the number of set bits in a bitmask by iterating through powers of two.
- `cleanup_and_release_resources`: Executes several conditional function pointers from a structure and calls routines to free memory and close library-related resources.
- `initialize_graphics_environment`: Opens required system libraries and allocates memory for display structures.
- `recurse_with_parameters`: Performs a series of recursive calls, including one call that passes a constant and a pointer to a global variable.
- `update_global_pointer`: Stores a specific global address into an offset of another global variable.
- `update_global_pointer_2`: Stores the value of one global variable into a specific offset of another global variable.
- `execute_conditional_logic`: Uses a global flag to prevent re-entry while calling another routine and potentially processing data from a global pointer.
- `execute_callback_and_set_flag`: Calls another routine with a constant and, if the result is non-zero, executes a subsequent routine before setting a global flag to one.
- `traverse_and_compare_list`: Traverses a linked list and updates a global variable if a value in a node exceeds the current stored value.
- `parse_character_buffer`: Checks a global flag, initializes specific variables if necessary, and then iterates through a character string to process each byte.
- `execute_minimal_calls`: Calls one routine and then calls another routine using a value from a global variable.
- `process_string_indices`: Iterates through a string and updates a table of values based on character comparisons and a global status.
- `select_and_process_table_entry`: Uses a masked parameter to select an entry from a table and, if valid and not already selected, updates a global pointer and calls another routine with data from that entry.
- `process_nested_data_structure_2`: Recursively traverses a hierarchical data structure by checking if entries are non-zero, clearing them, and conditionally resetting a global variable.
- `manage_memory_allocation`: Performs a series of memory management operations including allocation, field resetting, and string copying based on the result of an initial allocation attempt.
- `store_stack_data_to_structure`: Retrieves two values from the stack and stores them into a structure at specific offsets.
- `search_and_retrieve_short`: Searches for an entry using another routine and, if a specific bit is set in a byte at an offset, retrieves a short value from a calculated offset in a table.
- `process_nested_data_structure`: Traverses a hierarchical data structure by recursively calling itself to process child elements and linked list entries.
- `check_bit_status`: Searches for an entry using another routine and returns one if a specific bit is set in a byte at an offset, otherwise returns zero.
- `populate_linked_list_structure`: Traverses a linked list of data entries and populates a target structure with filtered elements.
- `initialize_metadata_values`: Populates two word-sized fields within a structure located via a pointer with specific constant values.
- `process_list_node_2`: Checks a pointer within a structure and, if not null, performs a recursive call with normalized fixed-point values.
- `search_table_for_index`: Iterates through a table of pointers to find a match for a provided value and returns the index.
- `decrement_list_value`: Searches for an index in a table and then recursively calls another routine with a decremented value from a structure.
- `increment_list_value`: Searches for an index in a table and then recursively calls another routine with an incremented value from a structure.
- `flag_table_entries`: Iterates through a table of pointers and checks specific conditions to either recursively call itself or set a flag in a global array.
- `iterate_and_flag_table`: Iterates through a table, clearing values and performing recursive calls based on specific conditions and pointer checks.
- `return_immediately_2`: This routine performs no operations and returns to the caller immediately.
- `clear_status_flag`: Checks a flag in a global array and, if set, performs a recursive call using data from a structure before clearing the flag.
- `clear_table_entry`: Finds an index in a table using a provided value, calls another routine with that index, and then clears a word at that index in a global array.
- `recursively_calculate_table_offset`: Recursively traverses a linked list of pointers until a null or invalid pointer is encountered.
- `update_structure_via_string_length`: Fetches a pointer from a table using an index, calculates a value based on the length of the string at that pointer, and passes this value and the pointer to another routine.
- `process_string_table_entry`: Retrieves a pointer from a table using an offset, calculates the length of the null-terminated string at that pointer, and calls another routine with a value derived from the string length and the pointer.
- `calculate_string_offset`: Calculates a value based on the length of a null-terminated string and calls another routine with the result.
- `execute_indirect_pointers`: Calls two different functions using pointers stored at specific offsets within a global variable and writes a longword value to a memory location.
- `perform_infinite_recursion`: An empty routine that calls itself indefinitely.
- `clear_offset_word`: Calls another routine and then stores a zero word at an offset from a value returned in a register.
- `trigger_field_reset`: Calls a routine that compares a value against a global variable and, if they match, resets that global variable and clears a specific field.
- `check_global_inequality`: Compares two global variables and calls another routine if they are not equal.
- `search_table_entry`: Iterates through a table of short words to find an entry that matches a provided parameter.
- `append_list_element`: Appends a new data structure to a linked list and initializes two internal fields within that structure.
- `update_entity_data_2`: Saves several registers to a stack or global area, updates specific bytes and words in a structure pointed to by a register, and restores them.
- `set_table_entry_flag`: Searches a table for an entry and sets a specific bit within a byte at a fixed offset.
- `invoke_callback_chain`: This routine invokes a callback function by retrieving a pointer from a data structure.
- `conditionally_execute_double_call`: Checks if a global variable is zero and, if so, calls another routine twice.
- `ensure_global_state_initialized`: Checks if a global variable is zero, and if so, calls another routine and stores its return value into that global variable.
- `clear_word_with_conditional_callback`: Checks if the third bit of a byte at a specific offset from a pointer is set.
- `toggle_metadata_state_flags`: Checks a global flag and, if zero, performs a series of bitwise operations on a global variable and updates several memory locations with specific values.
- `call_graphics_function`: Calls a function through a pointer stored at an offset from a global variable.
- `execute_conditional_callback_chain`: Checks a global variable to execute a function pointer at a specific offset.
- `store_stack_values_2`: Stores two values retrieved from the stack into a structure at specific offsets.
- `execute_conditional_function_pointers`: Checks two global variables and, if they are non-zero, executes the functions pointed to by the addresses stored in a table offset from a global variable.
- `execute_empty_stub`: This routine calls an empty stub and returns immediately to the caller without performing any operations.
- `allocate_memory_and_update_globals`: Attempts to allocate a memory block and, if the allocation fails, attempts to allocate it again using a different parameter.
- `update_fixed_point_values`: Calls a routine to perform a calculation and then updates two global variables by adding a parameter to their current values.
- `calculate_address_difference`: Subtracts one global address from another and returns the result.
- `initialize_global_values`: Stores two provided parameters into global variables and resets a third global variable to zero.
- `call_subroutine_zero`: Calls another routine with a zero value as an argument.
- `parse_character_stream`: Processes a stream of characters to build a buffer, handling delimiters and updating global counters.
- `call_routine_table_entries`: Walks a zero-terminated table of routine pointers, calling each one in turn until it reaches the zero entry.
- `invoke_global_callback`: The routine calls a function pointer stored in a global table, passing the address of a global variable as argument.
- `invoke_global_callback_2`: The routine calls a function pointer stored in a global table, passing the address of a global variable as an argument.
- `iterate_string_characters`: Iterates through every character of a string, calling another routine for each character, and returns the string length.
- `parse_format_string`: Iterates through a string to find percent signs and processes subsequent characters to handle integer formatting, pointer dereferencing, or string copying.
- `free_linked_list_and_memory`: The routine traverses a linked list to call a function pointer within each node and then iterates through a list of memory blocks to free them using a library call.
- `format_signed_integer`: Writes a minus sign through a pointer when the input value is negative, then repeatedly calls another routine to obtain digits and stores ASCII characters through pointers, and finally reverses the digit bytes in a stack buffer before terminating the string with a null byte.
- `parse_signed_integer`: The routine reads a signed decimal integer from a zero-terminated string pointed to by a register, handling an optional leading plus or minus sign.
- `initialize_data_structure_member`: Checks a bit flag in a structure and, if not set, calls a memory allocation routine to populate several fields within that structure.
- `traverse_linked_list_2`: Traverses a linked list while checking bit flags in a structure, performing calculations on values and updating a global variable based on the results.
- `load_stack_parameters`: Moves three values from the stack into registers d0, a0, and d1.
- `process_string_buffer`: Searches a buffer for a specific byte and copies characters while filtering out carriage return characters.
- `move_stack_to_stack`: Moves values from the stack into registers d0 and d1, then moves the value from the stack into a location on the stack.
- `update_status_flag_2`: Updates a global variable based on the result of a linked list search.
- `load_stack_to_register_2`: Moves a value from the stack into register d0.
- `validate_and_clear_structure_field`: Checks specific bit flags within a structure field and clears the word if certain conditions are met.
- `allocate_and_initialize_task`: Allocates a new entry in a linked list and copies a string into a newly allocated memory block.
- `store_parameter_to_global`: The routine stores a passed parameter into a global variable and returns one.
- `normalize_fixed_point_2`: Performs fixed-point division by normalizing the numerator and denominator through bit shifts and scaling.
- `normalize_fixed_point`: The routine takes two 32-bit inputs and normalizes a fixed-point value by scaling the numerator with a variable shift derived from the denominator’s magnitude, performing a division with remainder and a borrow correction, then returns a 64-bit result composed of a signed quotient and a scaled remainder.
- `calculate_complex_product`: Performs a series of arithmetic operations involving bit shifts and multiplications on two 32-bit values to produce a 32-bit result.
- `handle_game_event`: The routine is 21 instructions long but ends with an immediate return, performing no observable work on data or registers.
- `move_stack_to_register`: Moves a longword from a location on the stack into a data register.
- `execute_complex_callback`: Iterates through a linked list of structures, performing conditional function calls based on bitwise flags within each element.
- `parse_command_string_3`: Parses a string into a list of tokens, handling quoted substrings and expanding special escape sequences like newlines or carriage returns.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d5.
- `load_parameter_from_stack`: Copies a longword from a location on the stack into a data register.
- `manage_task_context`: Copies a string to a local buffer, performs conditional function calls based on task structure members, and updates a global pointer with a local stack address.
- `search_linked_list_2`: Traverses a linked list to find a specific node based on a provided count, updating a global variable with a status code and returning the resulting pointer.
- `manage_task_context_2`: Copies a string to a local buffer, performs conditional function calls based on task structure members, and updates a global pointer.
- `load_data_structure`: Loads several values from a pointer into global variables and extracts specific elements from a table based on an index.
- `modify_memory_and_register`: Increments a value at a pointer, performs a bitwise OR on a data register, and increments a byte at an offset from a table.
- `calculate_offset_value`: Calculates a value using a formula involving global variables and a table, then stores it in a pointer.
- `find_string_match`: Compares a target byte against each byte in a null-terminated string and returns a pointer to the first matching byte or null if no match is found.
- `find_byte_in_buffer`: The routine searches for a specific byte value within a memory buffer of a given length and returns a pointer to the matching byte or null if the length is exhausted without a match.
- `load_registers_from_stack`: Loads three values from the stack into two address registers and one data register.
- `append_string_to_buffer`: The routine appends a source string to the end of a destination string.
- `copy_bytes_with_termination`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered.
- `allocate_memory_block`: Searches a linked list of memory blocks to find a suitable free segment that can be split to satisfy a requested size.
- `load_stack_parameter`: Loads a longword from a location on the stack into a register.
- `unlink_and_free_memory_block`: Traverses a linked list of memory blocks to locate a specific block, modifies its size and flags, and then removes it from the list before calling a function pointer stored within the block structure.
- `load_stack_parameter_2`: loads a longword from the stack into a data register.
- `allocate_sub_block`: Searches a linked list of memory blocks to find a suitable free segment that can be split to satisfy a requested size.
- `process_memory_allocation`: Manipulates memory block pointers and sizes while interacting with system memory management routines to return a pointer to a memory segment.
- `decrement_global_value`: Decrements a global variable by one.
- `move_stack_to_d0`: Moves a long value from the stack into a data register.
- `process_linked_lists`: Traverses multiple linked lists to execute code pointers, check bit flags in structures, and perform conditional function calls.
- `increment_and_store_byte`: The routine increments the word pointed to by a pointer, increments the word at the offset of that pointer, and stores a byte from a data register into the address pointed to by the incremented word.
- `clear_byte_and_call`: Calls another routine with a passed parameter and a stack address, then sets the byte at the first pointer argument to zero before returning.
- `increment_buffer_offset_large`: Increments two values in a structure and stores a byte at an offset determined by a third value, repeating until a threshold is met.
- `check_structure_flags`: Checks specific bit flags within a structure and returns a value based on whether those bits are set.
- `increment_and_store_byte_recursive`: Increments two consecutive words in a structure and stores a byte at an offset calculated from the second word.
- `check_status_bits`: Evaluates a global variable against a bitmask and returns a specific status value.
- `execute_recursive_call`: Calls itself indefinitely based on the value of a passed parameter.
- `modify_memory_block_metadata`: Updates the flags, size, and pointers of a memory block structure based on provided parameters.
- `store_byte_and_update_state`: Examines status bytes in a structure and branches on their bits to set flags, reset counters, or return early.
- `move_stack_to_a0`: Moves a long value from the stack into an address register.
- `calculate_string_length`: Calculates a value by counting characters in a string, accounting for newline characters and structure flags.
- `reset_structure_fields_2`: Clears several fields within a structure, performs bitwise operations on a field based on a flag, and updates a value based on the result of another routine.
- `load_stack_parameters_2`: Loads four values from the stack into address and data registers.
- `copy_data_to_buffer`: Copies characters from a source to a destination buffer while checking structure flags and handling newlines.
- `copy_string_data`: Copies a sequence of bytes from a source buffer to a destination buffer based on a specified length.
- `manage_structure_state`: Traverses a linked list to check conditions or modifies various fields within a structure based on bitwise flags and integer comparisons.
- `load_stack_pointer_to_register`: Copies a value from a location on the stack into a data register.
- `validate_structure_state`: Checks the flags and contents of a structure to return a status code indicating success or failure.
- `parse_command_string_2`: Parses a string to determine specific character flags and updates a structure with calculated bitmasks and values.
- `traverse_linked_list_3`: Traverses a linked list of memory blocks and, if the list is empty, allocates a new block and initializes it with zeros.
- `parse_command_string`: Parses a string to identify a command and its associated numeric arguments.
- `parse_format_string_print`: The routine parses a format string to identify conversion specifiers and flags.
- `load_pointer_from_stack`: loads a value from a stack offset into a register.
- `execute_indirect_call_2`: Checks the result of a function call through a pointer stored in a global variable and conditionally calls another routine.
- `release_resource_and_cleanup`: Sets specific fields in a structure to default values and calls routines to free memory and remove a port.
- `copy_registers_to_table`: Copies the contents of several registers and stack values into a provided table.
- `enqueue_and_process_events`: Inserts a block of data into a linked list and iterates through the list, executing function pointers stored within the list elements until the list is empty.
- `initialize_library_environment`: Opens a library and executes a series of function calls from the returned library base using offsets while iterating through a null-terminated string.
- `move_registers`: Moves a longword from a location pointed to by the stack pointer into a register, then moves the contents of a data register into another data register.
- `tiny_move_ori`: Small routine of 2 instructions that copies a2 to d1, then merges 0 into d0.
- `execute_callback_chain_3`: Iterates through a sequence of function pointers stored in a structure, executing them while updating a stack-like pointer.
- `open_graphics_library`: Calls a function from a library base to initialize a graphics environment and stores the result in a global variable.
- `call_and_clear_global_pointer`: Checks if a global variable is non-zero, calls a function pointer located at an offset from that variable, and then clears the global variable.
- `open_intuition_library`: Calls a function from a library base to initialize an intuition environment and stores the result in a global variable.
- `execute_and_reset_callback`: Checks if a global variable is non-zero, calls a function pointer located at a negative offset from that variable, and then clears the global variable.
- `return_immediately_3`: The routine is empty and performs no operations before returning to the caller.
- `reset_input_state`: The routine calls another routine and then returns.
- `store_boolean_flags`: Tests specific bits in a register and stores the negated boolean results into several global variables and a pointer.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `clear_data_register`: Performs a bitwise OR operation on a data register with zero.
- `return_from_subroutine_2`: The routine is empty and performs no operations before returning.
- `fill_data_table`: Performs arithmetic on several values and uses nested loops to store a global variable and other values into a large memory table.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `execute_function_loop`: Retrieves a function pointer from a table using a bit-shifted index and executes it repeatedly until it returns zero.
- `tiny_moveq_lsr`: Small routine of 2 instructions that loads 2 into d4, then shifts d1 right by d4.
- `apply_bitmask_to_buffer`: Performs bitwise AND and OR operations using a mask and several global variables on specific offsets of a memory buffer.
- `execute_indirect_callback`: Retrieves a function pointer from a data structure and executes it.
- `execute_indirect_callback_2`: Retrieves a function pointer from a data structure and performs two consecutive calls to the same address retrieved from memory.
- `call_recursive_function`: The routine performs a recursive call to itself before returning.
- `append_data_to_linked_list`: Traverses a list of structures until a null pointer is found, then initializes a new structure at that location with several input values and specific byte constants.
- `reset_structure_fields`: Checks a bit in a byte at a specific offset to conditionally modify a global variable, then zeroes three words at specific offsets from a pointer.
- `clear_memory_word`: Checks a bit in a byte at a specific offset and, if set, calls another routine before zeroing out a word at the provided pointer.
- `update_status_byte`: Sets a byte at a specific offset in a structure based on an input parameter and a conditional check of another byte in the same structure.
- `store_byte_to_offset`: Stores a byte from a data register into a memory location at a fixed offset from a pointer held in a register.
- `copy_scaled_block`: Calculates a block size based on a value in a structure and calls a routine to perform a memory copy.
- `update_entity_state_logic`: Processes state transitions and counter decrements for an entity structure pointed to by a register.
- `iterate_string_null_terminated`: Recursively traverses a null-terminated string by calling another routine for each character.
- `multiply_value_by_constant`: Multiplies the lower sixteen bits of a register by a constant value and returns the result.
- `move_stack_to_register_2`: Moves a longword from a location relative to the stack pointer into a data register.
- `clear_structure_field`: Checks a value against a global variable and, if they match, clears the global variable and sets a field in a structure to zero.
- `append_to_linked_list`: Counts the elements in a list by traversing it, then appends a new set of data to the end of the list with a specific bit set in a field.
- `traverse_linked_structure_list`: Iterates through a linked list of structures, checking specific bit flags within each element.
- `modify_structure_flags_2`: Tests a bit in a structure, conditionally calls a routine, clears that bit, and performs a loop and a longword move.
- `validate_register_range`: Tests a word in a data register and branches if it is zero, not equal to three, or if it is any value other than three.
- `update_structure_fields`: Copies two words from a structure to a new location and calls another routine if a specific bit in a value is not set.
- `modify_structure_flags`: Compares a value against a global variable and sets a specific bit in a structure if certain bitwise conditions are met.
- `compare_and_branch_3`: Compares a word at an offset against two constants and branches accordingly, then performs a longword move and calls another routine.
- `apply_xor_mask_to_buffer`: Iterates through a buffer and applies bitwise XOR operations to specific memory offsets based on bitwise tests.
- `apply_bitmask_transform`: Iterates through a memory buffer to perform bitwise XOR and AND operations on specific offsets based on the values of sampled bytes.
- `update_bitmask_buffer`: Iterates through a memory buffer using a pointer, performing bitwise AND and OR operations on specific byte offsets to modify data based on a bitmask.
- `calculate_bit_shift`: Reads a word from a pointer and returns a value derived from shifting a constant by the lower three bits of that word.
- `update_state_from_table`: Searches a table for a matching entry and updates a global variable and a memory buffer based on the result and a bitmask check.
- `modify_bitmask_state`: Updates a global variable and a byte at a specific offset within a memory buffer using bitwise OR and AND operations.
- `process_list_changes`: Compares two global values and, if they differ, executes a routine based on a bitwise check of a structure field.
- `clear_structure_fields`: Checks a byte at an offset within a pointer-based structure and zeroes two global words if certain conditions are met.
- `select_data_by_flag`: Checks a bit in a structure to decide which offset to read from, then stores the result in a global variable.
- `update_entity_data`: Saves several registers to a stack or global area, updates specific bytes and words in a structure pointed to by a register, and restores the saved values.
- `check_and_update_state`: Tests two memory locations, loads a pointer from a global variable, extracts two words from that pointer into registers, clears a register, and calls another routine.
- `update_conditional_state`: Executes a function pointer from a global structure, performs bitwise operations on a value from a structure, and updates several global variables based on conditional checks.
- `update_state_and_branch`: Maps a value to specific constants in two global variables based on a short integer and performs conditional calls to other routines.
- `test_and_branch_routine`: Tests a byte at a specific memory location, calls another routine, and then performs an unconditional branch.
- `update_state_mapping_and_recurse`: Maps a value from a register to a specific pair of constants stored in two global variables.
- `check_state_threshold`: Compares a global value against a threshold derived from a table and executes another routine if they do not match.
- `clear_data_register_d0`: Performs a logical OR operation on a data register with zero to clear its contents.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `execute_callback_chain_4`: Repeatedly executes two function pointers stored in a global structure until the first one returns zero.
- `perform_recursive_call`: The routine performs a single call to itself before returning, resulting in an infinite loop of self-recursion.
- `call_bitmask_application_routine`: Calls a routine that performs bitwise OR operations between a source data table and two separate destination memory buffers.
- `call_append_list_data`: Calls a routine to append data to a list and then initializes two fields within the newly added structure.
- `append_list_data`: Calls a routine to append data to a list and then initializes two fields in the newly added structure.
- `invoke_structure_field_clear`: Calls a routine that compares a value against a global variable and, upon a match, resets that global variable and clears a specific field within a structure.
- `apply_bitmask_to_bitplanes_2`: Iterates through a source data table to perform bitwise OR operations on two separate destination memory buffers.
- `apply_bitmask_to_bitplanes`: Iterates through a source data table to perform bitwise OR and AND operations on two separate bitplane memory buffers.
- `process_event_callback_chain`: Iterates through a structure to evaluate specific status flags and command codes.
- `process_callback_chain`: Executes a series of function calls based on values in a structure, updating global variables and branching based on specific constant matches.
- `branch_on_comparison`: Compares a value at a specific offset to a constant and, if equal, calls another routine before updating a global byte and branching.
- `nullify_pointers_and_branch`: Clears register d0 and stores the resulting null pointer into two global variables.
- `update_conditional_global_value`: Stores a word from a pointer into a global variable and conditionally sets another global variable to a byte value based on a bit test.
- `copy_memory_block`: Copies sixteen bytes from a global variable to itself using a loop.
- `update_entity_state`: Modifies fields in a structure, performs recursive calls, and executes a chain of function pointers retrieved from a global variable.
- `execute_callback_loop`: Repeatedly executes a routine until it returns a specific value, then performs a final routine call and clears a global variable.
- `check_global_flag`: Checks if a global variable is zero and calls another routine if it is.
- `process_event_chain`: Calculates an offset, modifies a global variable based on a bit check, and executes a sequence of function pointers from a global structure.
- `initialize_global_state`: Checks a global variable and, if it is zero, calls another routine and updates the global variable with the return value.
- `compare_global_values`: Checks if two global variables are unequal and calls another routine if they are.
- `store_stack_data`: Retrieves values from the stack and stores them into a structure at specific offsets.
- `move_stack_parameter`: Moves a long word from the stack into a register.
- `compare_and_loop_through_table`: Compares a value against a table entry and iterates through a list using a decrementing loop.
- `check_conditional_execution`: Compares a value returned from a routine against a constant and conditionally executes another routine if they match.
- `find_table_entry`: Searches a table for a specific short value and returns the address of the matching entry.
- `execute_conditional_subroutines`: Calls another routine and performs a second call if the first routine returns a non-zero value.
- `clear_status_bit`: Searches a table for an entry and, if found, clears a specific bit in a byte at an offset within that entry.
- `modify_table_entry_flags`: Searches for a table entry and, if found, modifies specific bits in a byte at a fixed offset within that entry.
- `update_table_entry`: Searches a table for an entry and, if found, writes two provided values into that entry at a specific offset.
- `check_status_bit`: Searches a table for an entry and returns one if a specific bit is not set in a byte within that entry, otherwise returns zero.
- `execute_callback_chain_5`: Invokes a routine through a pointer retrieved from a data structure.
- `initialize_metadata_structure`: Populates several global variables with metadata and performs bitwise operations on a global flag.
- `call_subroutines_conditionally`: Checks a global variable and calls another routine twice if the variable is zero.
- `store_byte_and_branch`: Stores a byte into a memory location and then performs a short branch.
- `process_savegame_string_input`: Parses a null-terminated string to calculate text display metrics, such as character widths and line breaks, while updating a character buffer and cursor position.
- `parse_save_data_string`: Iterates through a character buffer to calculate text display metrics, such as line breaks and character widths, based on specific character types.
- `process_text_input_buffer_3`: Manages a character buffer by handling backspace and carriage return inputs while updating text display metrics and cursor positions.
- `process_text_input_buffer_2`: Manages a character buffer by handling backspace and carriage return inputs, while updating text display metrics and cursor positions based on character properties.
- `branch_to_address_2`: Performs a short branch to a specific address.
- `call_subroutine_with_offset`: Tests a word at a pointer location and, if it is non-zero, calculates a new value by adding a word from another pointer to zero and calls another routine with this value on the stack.
- `process_text_input_buffer`: Handles character input by updating a buffer with backspace and carriage return logic.
- `process_text_input`: Processes a stream of character input into a buffer, handling backspaces and carriage returns.
- `process_event_chain_variant`: Executes a sequence of function pointers from a global structure within a loop and performs arithmetic on a global variable.
- `process_complex_state`: Performs conditional logic based on a return value, clears several global variables, and enters a loop that executes a chain of function pointers from a global structure.
- `adjust_global_offset`: Calls another routine and then updates a global variable by adding the difference between two other global values to it.
- `process_message_loop_2`: Iterates through a loop that calls routines via pointers stored at offsets from a global variable until a zero value is returned.
- `execute_unreachable_branch`: This routine consists of an infinite loop that is never reached during normal execution.
- `process_state_logic`: Compares a value at a memory offset against several constants to update global variables or perform arithmetic before calling another routine.
- `decrement_and_branch_2`: Subtracts a constant from a long value at a memory location and then performs a relative branch.
- `compare_and_set_value`: Compares a value at a memory offset to a constant, and if it is greater than or equal to that constant, calculates the difference and updates a value in a structure.
- `compare_and_set_flag`: Compares a value at a memory offset to a constant, and if it is greater than or equal to that constant, calculates the difference and sets a bit in the condition code register.
- `rotate_byte_value`: Swaps a byte at a specific offset within a structure with its neighbor using a temporary register.
- `calculate_and_loop_indices`: Performs arithmetic on a returned value to determine a loop count and then iterates through a range to call another routine.
- `set_status_bits_in_loop`: Iterates through a range of values, searching a table for each, and sets a specific bit in the found entries.
- `parse_string_data`: Parses a string by iterating through characters and performing library calls to convert segments into a numeric value stored in a global variable.
- `call_routine_and_halt`: Calls another routine with a pointer to a global variable and then enters an infinite loop.
- `update_data_structure_2`: Writes several values into a memory structure at specific offsets relative to a pointer retrieved from a global variable.
- `serialize_game_state_linked_list`: Iterates through a linked list of structures to calculate the total element count and then writes their contents into a destination buffer.
- `initialize_game_data_structures_3`: Populates various global tables and memory structures by iterating through loops to assign values, including bitmask-based property assignments.
- `initialize_resource_metadata`: Parses a header from a pointer-based data structure to extract configuration flags, offsets, and lengths.
- `traverse_and_process_recursive_structure`: Initializes a block of memory with zeros and performs a depth-first traversal of a data structure.
- `store_byte_to_memory`: Stores a single byte into a memory location.
- `call_load_r_g_b4`: Library stub: loads its arguments and the graphics library base, calls LoadRGB4 and returns its result to the caller.
- `store_passed_parameters`: Stores seven passed parameters into various global variables.
- `call_load_r_g_b4_2`: Library stub: loads its arguments and the graphics library base, calls LoadRGB4 and returns its result to the caller.
- `execute_recursive_call_2`: This routine performs a recursive call to itself.
- `install_recursive_exception_handler`: Fetches function pointers from a table to execute specific routines and then installs itself as an exception handler by updating a pointer within a retrieved structure before calling itself recursively.
- `clear_register_bits`: Performs two bitwise OR operations with zero on a register.
- `clear_register_bit`: Performs a bitwise OR operation with zero on a register.
- `set_global_bit`: Sets a specific bit in a global variable using a bitwise OR operation.
- `clear_global_bit`: Clears a specific bit in a global variable using a bitwise AND operation.
- `execute_indirect_function_pointers`: Calls two different functions via pointers stored at offsets within a global variable.
- `execute_empty_call`: The routine consists of a single call to another routine and then returns.
- `dispatch_exception_handlers`: Fetches function pointers from a table and executes them sequentially.
- `move_longword`: Copies a longword from a location pointed to by a register into another register.
- `initialize_disk_error_state`: Modifies global variables and executes a sequence of subroutine calls based on memory values.
- `execute_sequenced_calls`: Conditionally executes another routine twice with different values assigned to a global variable and performs a bitwise AND on a global variable.
- `execute_range_operations`: Calls functions through a table of pointers and performs calculations to update bounds across a range of memory.
- `toggle_global_bit`: Sets the high bit of a global variable, calls another routine, and then clears the high bit.
- `clear_table_entries`: Calculates an offset into a table using a parameter and clears three consecutive longwords at that location.
- `copy_string_to_buffer_2`: Copies a null-terminated string into a global buffer and updates a global pointer using an index.
- `execute_function_via_pointer`: Selects a pointer to a global variable and calls a function through a pointer retrieved from a memory location.
- `fetch_table_entry`: Uses values from a data structure to calculate an offset and retrieve a byte from a table.
- `fetch_table_entry_2`: Retrieves a byte from a table by calculating an offset based on values stored in a data structure.
- `process_message_loop`: Iterates through messages using system library calls and performs conditional execution of functions stored in a table.
- `parse_numeric_string`: Initializes a global variable to a specific value and calls a routine to convert a string into a numeric value stored in that variable.
- `store_ascii_digit`: Converts a byte to its ASCII digit equivalent and stores it in a global array at a specified index.
- `convert_and_store_value`: Calculates a numeric value, converts it into a string of ASCII characters, and updates several global variables and a memory location via a pointer.
- `set_flag_on_condition`: Sets a global variable to one if another global variable contains specific values and a third variable is zero.
- `execute_complex_sequence`: Sets a bit in a global variable, stores two parameters into global locations, calls several other routines, and updates a global offset value.
- `decompress_bitstream_data`: Iterates through a bitstream to reconstruct bytes by performing bitwise shifts and logical operations.
- `decrement_and_shift`: Decrements a global variable and returns a value from a pointer with the most significant bit set and the bits shifted right.
- `process_bit_shifts`: Iterates through a memory block, performing bitwise right shifts and setting the most significant bit on the resulting values.
- `update_and_copy_data`: Updates a global variable based on its current value and performs a block memory copy to a destination pointed to by a global variable.
- `shift_memory_block`: Checks a global variable and performs a block memory move to shift data based on a calculated offset.
- `copy_memory_range`: Copies a block of data from one memory location to another and updates a global variable.
- `clear_data_byte_2`: Performs a logical OR operation on a byte in a register with zero.
- `clear_data_register_byte_2`: Performs a logical OR operation on a byte in a data register using zero to clear its value.
- `clear_data_register_byte_3`: Performs a logical OR operation on a data register using a zero immediate value.
- `zero_data_register_byte`: Performs a logical OR operation on a byte in a data register with zero.
- `check_memory_availability`: Queries available memory and updates several global variables based on the results of memory allocation checks.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `copy_global_values`: Copies values from four global variables to four other global variables.
- `copy_memory_block_2`: Copies two sets of two 16-bit values from one location in memory to another.
- `call_stored_functions`: Checks two global variables and calls the functions pointed to by them if they are non-zero.
- `assign_pointer_value`: Copies the value from one global variable to another.
- `clear_data_register_2`: Performs a logical OR operation on register d0 with zero.
- `clear_data_register_byte_4`: Performs a logical OR operation on a byte in a data register using a zero value.
- `execute_function_list_logic_2`: Iterates through a list of function pointers and executes them based on specific bit flags and conditions.
- `process_list_ranges`: Iterates through two separate loops to traverse lists and update global values based on calculated offsets and conditions.
- `execute_function_list_logic`: Moves a long value from the stack to a data register, evaluates a global variable against a bitmask, and iterates through a list of function pointers while performing comparisons and range checks.
- `execute_function_list`: Iterates through a list of function pointers, performs several long integer comparisons, and checks range bounds against a table.
- `iterate_function_list`: Checks specific bit flags and iterates through a list of function pointers, executing them until a null pointer or a specific condition is met.
- `calculate_address_offsets`: Calculates several memory addresses based on values in registers a0 and d0 and stores them in a1.
- `compare_long_values`: Compares a long integer in one register against a long integer in another and branches if the first is greater.
- `check_range_bounds`: Compares a value in a register against a minimum and maximum value stored in a table.
- `clear_table_entries_2`: Iterates through a table and clears three consecutive integer entries to zero if they fall within a specified range.
- `register_interrupt_handler`: Registers an interrupt handler with the operating system.
- `execute_callback_chain_2`: Checks a global variable to execute a function pointer and performs conditional updates to other memory locations via function calls.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `copy_memory_blocks`: Performs a complex block memory copy from a source to multiple destination offsets using nested loops.
- `update_linked_list`: Sets a bit in a global variable and iterates through a list of structures to increment a value if a specific pointer matches a global variable.
- `return_immediately_4`: This routine performs no operations and returns to its caller immediately.
- `clear_fifth_bit`: performs a bitwise AND operation on a global variable to clear the fifth bit.
- `update_bitmask_buffer_2`: Calculates a bitmask from a sequence of encoded bytes and applies it to a destination buffer using bitwise OR operations.
- `modify_register_and_branch`: Subtracts a value from a register, performs conditional branches, modifies a register, and clears a bit in another register.
- `apply_bitmask_to_buffer_2`: Iterates through a source table and applies bitwise OR operations to a destination buffer using shifted bitmask values.
- `return_stack_value`: This routine is empty and simply returns the value currently residing at the top of the stack.
- `find_closest_coordinates`: Iterates through a list of tables to find the entry with the minimum calculated distance from two input parameters and stores the resulting index and value into global variables.
- `update_indexed_values`: Decrements a counter in a register and, if not zero, performs a loop to store two word values from registers into a table using an offset derived from the stack.
- `update_parameter_table`: Checks a global variable and a flag before either waiting for another global variable to reach zero or storing a passed parameter into a table at an offset determined by the first parameter.
- `wait_for_condition`: Enters a loop that waits for a specific global variable to reach zero.
- `update_table_data`: Calls another routine to write values into a memory block based on a calculated index.
- `compare_and_return_registers`: Calls another routine and returns the concatenated values of two registers.
- `modify_global_with_parameter`: Sets a bit in a global variable, assigns a parameter to a global variable, calls another routine, and then clears a bit in the global variable.
- `clear_data_byte_3`: Performs a logical OR operation on a data register with zero.
- `call_clear_and_advance_pointer`: Calls a routine that updates a global variable and iterates through a list of words, then returns the concatenated contents of two 32-bit registers.
- `handle_event_loop`: Manages a loop that processes events and executes callback routines based on a provided parameter.
- `process_message_callback_loop`: Iterates through a loop that executes a routine via a pointer from a global variable and checks its return value.
- `reply_and_compare`: Calls a routine via a pointer from a global variable, then compares a long value at a specific memory location against a constant.
- `update_global_pointer_3`: Stores a parameter into a global variable and calls another routine before returning two values held in data registers.
- `initialize_state_sequence`: Sets a global flag and a series of character constants before calling other routines and updating global variables.
- `reset_state_sequence`: Updates a global flag and character constants, then calls other routines to reset a specific global variable.
- `search_table_for_value`: Traverses a table of short integers to find a specific value, returning a combined 32-bit result based on the search outcome.
- `update_global_pointer_4`: Compares a register against a constant and, if they are equal, stores that register's value into a global memory location.
- `process_table_indexed_data`: Calculates an index from a value, retrieves data from a table, and performs a series of memory copy operations to rearrange or transfer blocks of data based on calculated offsets.
- `divide_and_call`: Divides a value in a register by one hundred, calls another routine, and then branches to another location.
- `iterate_table_entries`: Uses a loop to compare a value against entries in a table, incrementing a pointer by a fixed amount for each non-matching entry.
- `update_pointer_list`: Copies a long value from one memory location to another and performs a comparison between two values to determine a branch destination.
- `copy_memory_blocks_2`: Calculates destination and source addresses using table lookups and performs a nested loop to copy word data between memory regions.
- `update_table_entry_2`: Calculates an index from a numeric value to locate a table entry, then writes several values into a null-terminated sequence of memory blocks.
- `execute_graphics_initialization`: Toggles a global flag, updates a pointer in a structure, and calls several routines via pointers, including a loop that checks a value against specific bounds.
- `load_graphics_view`: Calls two functions via global pointers, one of which is used to load a graphics view and another to manipulate a graphics coprocessor.
- `clear_d0_register`: Performs a logical OR operation on the data register zero with zero to clear its contents.
- `call_indexed_function`: Loads a word from a pointer, calculates an address using a table and an offset, and calls the routine at that address in a loop.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `signal_and_increment_counter`: It checks a global variable, calls a function via a pointer offset, increments a global byte, and signals a semaphore.
- `process_recursive_state_flags`: Checks multiple bits in a global variable to determine whether to execute a function pointer or perform a recursive call.
- `toggle_and_recursively_check_flags`: Sets a high bit in a global variable, calls itself, and then clears that bit while setting a different bit.
- `decrement_and_increment_pointer`: Loads a word from a memory location, decrements it, and stores the result back into that location.
- `call_via_pointer_list`: Loads three values from a pointer, stores a pointer to a memory location, calls a routine using the second loaded value, and restores the stored pointer.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `write_table_entry`: Searches for a null terminator in a table and writes four values into the memory location immediately following it.
- `copy_list_elements`: Copies a list of 6-byte structures to a new location while decrementing a global variable.
- `fetch_list_data`: Calls another routine and then loads a long value and a word value from a memory location pointed to by a register.
- `compare_memory_blocks`: Compares two blocks of memory using a loop and branches based on the result.
- `compare_pointer_values`: Compares two memory addresses and branches based on whether they are equal.
- `initialize_global_registers`: Sets two global variables to specific values.
- `copy_and_increment_data`: Copies a fixed number of words from a global variable to itself and then increments a global counter.
- `verify_and_fetch_data`: Compares two sequences of words and, if a mismatch is found, searches a table to update global variables and calls another routine.
- `copy_linked_list_nodes`: Traverses a structure of linked nodes, copying data between them, and performs a bitwise AND on a global variable.
- `branch_to_previous`: Performs a short relative branch to a previous instruction location.
- `clear_bits_in_global`: Performs a bitwise AND operation on a global variable to clear specific bits.
- `process_table_index_search`: Depending on a header value, this routine either searches a table for a calculated coordinate value and stores the iteration count into a destination, or calculates new coordinates from a source table and writes them into a global array.
- `search_and_store_index`: Searches a table for a calculated value and stores the number of iterations performed into a location.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 25 into (a1)+, then merges 25 into (a1)+.
- `parse_and_initialize_data_structure`: Reads a header from a pointer-based data structure to extract configuration flags, offsets, and lengths.
- `copy_structured_data_buffer`: Iterates through a source buffer to copy bytes into a destination buffer at fixed intervals.
- `swap_and_update_global_state`: Swaps values between a global variable and a series of memory locations before entering nested loops.
- `decrement_and_call_loop`: Repeatedly calls another routine while decrementing a value derived from a register.
- `transform_data_values`: Performs complex arithmetic and bitwise manipulations on input data to produce two resulting values.
- `call_subroutines_repeatedly`: Calls the same routine five times in succession.
- `traverse_and_modify_buffer`: Iterates through a memory buffer using a pointer, performing arithmetic on a counter and updating values based on signed byte comparisons.
- `translate_data_table`: Uses a source buffer and a lookup table to translate bytes into new values and writes them to a destination memory area.
- `initialize_pointer_array`: Calculates addresses by adding offsets from a pointer to a series of global variables and sets several globals to a constant value.
- `expand_data_buffer`: Uses a nested loop to write bytes into a destination buffer by looking up values in a table based on source data.
- `copy_buffer_with_stride`: Iterates through a source memory buffer and writes bytes to a destination buffer using a specified stride or offset.
- `update_data_structure`: Updates a series of values in a memory block by adding words retrieved from a pointer.
- `search_for_sentinel_word`: Iterates through a list of code pointers, comparing each entry against a global constant until a match or a null value is found.
- `modify_global_state`: Checks a value from a pointer and updates two global variables based on a modulo operation and a non-zero condition.
- `compare_table_entry`: Loads a pointer from a table, calculates an offset, and compares a word from the resulting address against a global variable.
- `return_without_action`: The routine performs no operations and immediately returns to its caller.
- `increment_offset_value`: Adds a word from a pointer to a specific offset within a memory block and increments a global counter.
- `increment_secondary_offset`: Adds a word from a pointer to a different offset within a memory block and increments a global counter.
- `insert_into_sorted_linked_list`: Inserts a new record into a sorted linked list by traversing the list to find the correct position, shifting subsequent elements to accommodate the new entry, and incrementing a global counter.
- `insert_into_sorted_list`: Inserts a set of values into a linked list structure while maintaining order and incrementing a global counter.
- `increment_global_and_store`: Stores a word into a memory location and increments a global variable.
- `tiny_move_rts`: Small routine of 2 instructions that copies (a0)+ to (a0)+, then returns.
- `decrement_value_at_offset`: Reads a word from a pointer, uses it as an offset to find another word, and decrements that word if it is not zero.
- `fetch_conditional_word`: Fetches a word from a table and, if the result is zero, attempts to fetch another word from a different location.
- `check_table_value`: Checks a value at an offset from a pointer and conditionally calls another routine based on whether that value is zero.
- `check_null_pointer`: Checks if the word at the location pointed to by a register is zero and calls another routine if it is.
- `check_non_null_pointer`: Checks if the word at the location pointed to by a register is non-zero and calls another routine if it is.
- `check_offset_word`: Checks if the word at an offset of two bytes from a pointer is zero and calls another routine if it is.
- `check_offset_word_and_call`: Checks if the word at a two-byte offset from a pointer is zero and calls another routine if it is.
- `fetch_scaled_table_value`: Uses a word from a pointer to index into a table and returns the resulting value shifted left by one bit.
- `return_word_from_pointer`: Returns the word stored at the location pointed to by the input register.
- `copy_list_elements_2`: Iterates through a list of 8-word entries and copies data from one offset to another within each entry.
- `skip_zero_word`: Checks a word at a memory location and increments a pointer by eight if the word is not zero.
- `copy_structure_and_pointer`: Copies a word and a following pointer from one location to another, then stores a global variable into the destination.
- `process_table_matches`: Searches a table for an entry matching a value in a register, then performs data shifts within matching entries and resets a global variable if a specific match is found.
- `increment_register_and_branch`: Adds eight to a word in a register and performs a short branch to another location.
- `reset_global_on_match`: Compares a word from a pointer to a global variable and resets that global variable to zero if they are equal.
- `lookup_table_and_iterate_list`: Calculates an index into a table based on a value divided by one hundred, retrieves a pointer from that table, and then iterates through a list of words until a specific value is matched.
- `compare_and_branch_2`: Loads two pointers from memory, calculates new addresses based on offsets, and performs a comparison to determine whether to branch or increment a pointer.
- `store_stack_values`: Stores two values from the stack into two specific global variables.
- `process_word_and_branch`: Loads a word from an offset of two bytes from a pointer into two registers and calls another routine.
- `update_indexed_table`: Searches a table for a specific value and writes several related words into a null-terminated sequence of memory blocks.
- `test_word_and_offset_branch`: Loads a word from a pointer and branches if it is zero; otherwise, it updates the pointer with an offset and performs an unconditional branch.
- `search_and_copy_structure`: Copies data from one structure to another and searches a list for a matching value before potentially calling another routine.
- `store_stack_value`: Stores a single value from the stack into a global variable.
- `store_multiple_values_to_table`: Uses a word from a pointer to calculate an offset and stores four consecutive words into a table and a separate memory area.
- `clear_and_advance_pointer`: Sets a bit in a global variable, clears a word in a global variable, and then iterates through a list of words by clearing them or advancing a pointer.
- `clear_word_and_advance`: Loads a word from a pointer and branches if it is zero, otherwise clears that word and branches to another location after advancing the pointer by eight bytes.
- `clear_linked_list`: Traverses a list of structures to copy data and clears several memory regions if a specific bit in a global variable is set.
- `branch_to_routine`: Performs a short branch to another routine.
- `check_and_clear_pointer`: Tests a specific bit in a global variable and, if set, clears the value at a different global pointer location.
- `reset_global_pointer`: Checks a global pointer and, if it is not zero, clears the value at that location and branches to another routine.
- `clear_low_nibble_bits`: Performs a bitwise AND on a global variable to clear specific bits.
- `tiny_addq_rts`: Small routine of 2 instructions that adds 8 to a0, then returns.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `store_word_to_global`: Stores a word from a pointer into a global variable.
- `copy_word_to_globals`: Copies a word from a location pointed to by a register into two consecutive global word locations.
- `nullify_routine_pointer`: Calls another routine and then sets a global pointer to zero.
- `set_byte_value`: Stores a single byte into a memory location.
- `clear_global_array`: Fills a series of words in a global array with zero using a loop.
- `return_concatenated_registers`: Concatenates two 32-bit registers and returns the resulting 64-bit value.
- `clear_bit_in_long`: Clears the seventh bit of a long value at a specific memory location.
- `branch_if_zero`: Branches to another routine if a global variable is zero.
- `set_bit_in_long`: Sets a specific bit in a longword located at a fixed memory address.
- `copy_memory_block_3`: Copies a series of words from one memory location to another if two global conditions are met.
- `update_global_pointers`: Reads two words from a pointer, stores them in global variables, and conditionally increments a global counter.
- `call_stub_2`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `fill_buffer_with_colors`: Calculates a destination address based on table lookups and fills a large memory area with specific word values determined by bitmasking an input.
- `fill_pattern_buffer`: Fills a memory area with a repeating pattern of five word values based on dimensions stored in a table.
- `clear_global_variable`: Sets a global variable to zero and calls another routine with an incremented value.
- `call_store_word_at_index`: Calls a routine that stores a word into a global table at an offset determined by a bit-shifted index.
- `fetch_and_store_word`: Retrieves a word from a global table using a bit-shifted index and then stores a word into a global table at a bit-shifted offset.
- `update_table_word`: Fetches a word from a global table using a bit-shifted index and stores a word into a global table at a bit-shifted offset.
- `compare_and_call`: Compares a returned value against a value at a pointer and calls another routine if they do not match.
- `fetch_fallback_word`: Attempts to fetch a word from a table and, if the returned value is zero, attempts to fetch another word from a different location.
- `recursively_test_table_bit`: Calls itself recursively if the value returned by a previous call is non-zero.
- `modify_table_bit_2`: Calls a routine to modify a specific bit within a word located in a global table using a bitmask.
- `call_modify_table_bit`: This routine acts as a wrapper that calls a function to modify a specific bit within a word in a global table.
- `pass_pointer_arguments`: Passes two consecutive words from a pointer as arguments to another routine.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies (a0)+ to d0, then returns.
- `return_parameter`: An empty routine that simply returns the value passed in a register.
- `tiny_addq_rts_2`: Small routine of 2 instructions that adds 4 to a0, then returns.
- `read_offset_word`: Reads a word from an offset of four bytes from a pointer held in a register.
- `add_table_entry_values`: Searches a table for a specific short value and, if found, adds the values of the matching entry to another memory location.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `call_graphics_function_2`: Calls a function pointer located at an offset from a global variable.
- `call_stub_3`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `process_data_list`: Searches through a list for a matching value and performs conditional data copying and list manipulation based on the result.
- `decrement_and_call_function`: Iterates through a table to subtract specific bit-pattern values from each word before calling a function pointer located at an offset from a global variable.
- `compare_memory_blocks_2`: Calls a routine to compare two blocks of memory.
- `update_accumulated_values`: Retrieves a word and its successor, then adds them to existing values at specific offsets in a destination structure.
- `clear_global_array_2`: Clears a series of five words in a global memory area by setting them to zero.
- `copy_table_entries`: Reads words from a source pointer and stores them into a destination table at calculated offsets.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `add_to_global_offset`: Retrieves a word and adds it to a value stored at an offset within a structure, then increments a global counter.
- `store_fetched_word`: Calls another routine to fetch a word and stores the result into a destination offset from a pointer, then increments a global variable.
- `store_fetched_word_offset_eight`: Calls another routine to fetch a word and stores the result into a destination offset of eight from a pointer, then increments a global variable.
- `fetch_and_store_word_2`: Performs a sequence of fetching and storing words into a global table using bit-shifted indices.
- `update_indexed_table_entry`: Fetches a word from a global table using a bit-shifted index and stores it into another global table at an offset determined by a bit-shifted index.
- `decompose_table_differences`: Iterates through a table of values, calculating the difference between consecutive elements.
- `split_difference_values`: Calculates the difference between two consecutive values in a table and stores the halved difference and the remainder into another table.
- `clear_memory_and_decrypt_table`: Clears two large memory blocks by zeroing out specific offsets and then iterates through a linked list to reorganize data into a contiguous block.
- `reorganize_table_and_decrypt_data`: Copies a linked list of data structures into a contiguous block of memory and then performs a bitwise subtraction on a series of words in a table to transform their values.
- `branch_to_address`: Performs a short relative branch to a specific address.
- `process_table_and_execute`: Iterates through a table to subtract bit-pattern values from each word and then executes a function pointer from a global variable.
- `fetch_scaled_word`: Returns a word from a global table using an index that is multiplied by two.
- `fetch_word_at_index`: Returns a word from a global table using a bit-shifted index.
- `store_word_at_index`: Stores a word into a global table at an offset determined by a bit-shifted index.
- `get_bit_from_table`: Retrieves a word from a table and returns a value based on a specific bit extracted from the input.
- `modify_table_bit`: Modifies a specific bit within a word located in a global table using a bitmask.
- `validate_table_pointer`: Compares a pointer retrieved from a table against a null value or another pointer value.
- `validate_table_pointer_2`: Compares a value retrieved from a table through a pointer against a global constant or another pointer's value.
- `compare_table_entry_2`: Retrieves a value from a table using an index and compares it against a second value.
- `reset_hardware_registers`: Sets several global variables to zero and then sets a specific bit in one of them.
- `update_dma_control`: Sets several global variables to zero and then clears a specific bit in a final global variable.
- `populate_lookup_table`: Calculates a series of values using division and modulo operations and stores them as words in a table.
- `test_and_branch`: Tests a word at a specific memory location and branches if it is equal to zero.
- `reset_audio_and_globals`: Iterates through a data structure to clear several memory locations and resets various global variables and audio volume registers.
- `reset_audio_volumes`: Sets four consecutive global variables to zero and then sets a final global variable to a specific value.
- `increment_and_check_pointer`: Increments a long value at a global address, compares the new byte value at that address against a constant, and clears the global value if the comparison fails.
- `update_audio_registers`: Modifies audio hardware registers and updates a global variable using clamping logic within a loop.
- `retrieve_table_value_via_index`: Calculates an index using a value from a global variable and a bitwise operation on a byte from a structure, then iterates through a table to find a word that satisfies a comparison against a value in that structure.
- `search_table_for_value_3`: Iterates through a table to find a value that is greater than or equal to a specified word and stores a value from a calculated offset into a destination.
- `store_word_at_offset_2`: Stores a word from one pointer's offset into another pointer's offset.
- `search_table_for_value_2`: Iterates through a table to find a value that is greater than or equal to a specified word.
- `store_word_at_offset`: Stores a word into a memory location at a fixed offset from a pointer.
- `initialize_audio_hardware`: Clears a byte in a data register, parses a DMA control block, and manipulates audio hardware registers before entering an infinite loop.
- `parse_dma_control_block`: Extracts a nibble from a source word to index into a table, then populates a destination structure with calculated offsets, lengths, and bit-shifted values.
- `process_dma_control_block`: Updates a data structure by calculating offsets and bit-shifting values for a DMA transfer.
- `configure_dma_transfer`: Sets up a data structure for a transfer, modifies a global variable, and enables DMA by writing to the DMA controller register.
- `initialize_audio_dma`: An infinite loop that follows attempts to manipulate audio hardware registers and DMA control.
- `update_status_flag`: Compares a word with a value from a pointer and sets or clears a byte at a fixed offset based on the result.
- `clear_byte_at_offset`: Stores a zero byte into a location at a fixed offset from a pointer.
- `update_clamped_accumulator`: Updates a value by adding or subtracting a byte based on a flag, then clamps the result against a limit and clears the limit if the threshold is reached.
- `clamp_and_store_value`: Subtracts a word from a value at an offset, compares it against another value, and stores the result or a cleared value into a new location.
- `update_score_and_flags_2`: Calculates a value using nibbles from a structure and a table, adds it to a global variable, and updates a byte in a structure.
- `update_score_and_flags`: Writes a short value derived from two inputs to a global variable and updates a byte in another global structure by combining a nibble from a neighboring byte with the existing byte.
- `copy_word_to_offset`: Copies a word from one memory location to another location at a fixed offset from a pointer.
- `modify_clamped_value`: Updates a 12-bit value stored in a structure by adding or subtracting a byte based on a control nibble, applying clamping logic to keep the result within specific bounds.
- `update_clamped_half_value`: If the high nibble of a byte is non-zero, it adds that nibble to a word and clamps the result to a maximum of sixty-four; otherwise, it subtracts the low nibble from the word and clamps the result to zero.
- `decrement_and_clamp_value`: Subtracts a byte from a word, ensures the result is not negative, and stores half of the resulting value into a global variable.
- `subtract_and_clamp_word`: Subtracts a byte from a word, clamps the result to a minimum threshold, and stores the lower twelve bits of the result into a global variable.
- `add_and_clamp_word`: Adds a byte to a word, clamps the result to a maximum threshold, and stores the lower twelve bits of the result into a global variable.
- `branch_on_byte_value`: Compares a byte from a pointer against several constant values and branches to different locations based on the match.
- `invert_global_word`: Performs a bitwise NOT operation on a global variable.
- `invert_byte_from_pointer`: The routine reads a byte from an address computed from an uninitialized pointer plus a small offset, subtracts one from it, then stores the bitwise NOT of the result into a global variable.
- `modify_and_shift_byte`: If a byte at a specific offset is greater than a certain character value, it is replaced with a constant.
- `update_global_with_mask`: The routine checks if the lower five bits of a byte at a specific offset are non-zero.
- `execute_infinite_loop`: Enters an infinite loop that performs no actions.
- `execute_infinite_loop_2`: The routine enters an infinite loop that prevents further execution.
- `execute_infinite_loop_3`: Enters an infinite loop that prevents further execution of the current thread.
- `clear_register_d0`: Performs a logical OR operation on register D0 with zero.
- `perform_unconditional_branch`: Performs a bitwise OR on a register and then executes an unconditional branch.
- `call_close_work_bench`: Library stub: loads its arguments and the intuition library base, calls CloseWorkBench and returns its result to the caller.
- `call_get_prefs`: Library stub: loads its arguments and the intuition library base, calls GetPrefs and returns its result to the caller.
- `call_indirect_via_offset`: Calculates an address by adding a negative offset to a global variable and then calls the function located at that address.

### Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 1 of 9).adf:iconx

- `initialize_application_environment`: Prepares the execution environment by restoring patch states and determining whether to launch via the command line or the Workbench.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `dispatch_cli_or_workbench_start`: Chooses between a command-line start and a Workbench start.
- `build_wait_and_endcli_command`: Builds a command string from a template and the given name, including a wait command with a long timeout and an endcli redirected to NIL, after checking whether the name starts with a dot.
- `install_library_functions`: Creates and installs a library from a descriptor: it makes the library structure, sets up its function table, and returns the result.
- `scan_ram_entries_for_names`: Allocates a 260-byte buffer and walks a linked list of entries, copying the entries whose name bytes are above the control range into it and passing them to helper routines.
- `concatenate_and_pass_buffer`: Copies a null-terminated string into a destination buffer, appends a second null-terminated string to that same buffer, and then calls another routine with the resulting buffer and the total combined length.
- `count_string_length`: Receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminating zero.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `append_string_end_and_call`: Advances a pointer to the terminating zero of a string, then passes that position and a second argument to a helper routine.
- `parse_signed_decimal`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts_3`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_via_pointer`: Stores a byte from a register into a memory location pointed to by another register.

### Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 1 of 9).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 2 of 9).adf:bootblock

- `boot_entry_2`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 3 of 9).adf:bootblock

- `boot_entry_3`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 4 of 9).adf:bootblock

- `boot_entry_4`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 5 of 9).adf:bootblock

- `boot_entry_5`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 6 of 9).adf:bootblock

- `boot_entry_6`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 7 of 9).adf:bootblock

- `boot_entry_7`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Simon the Sorcerer (1993)(Adventure Soft)[cr PDX](Disk 9 of 9).adf:bootblock

- `boot_entry_8`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.
- `step_position_state_machine_2`: Updates copper list pointers and moves a position by four units in a direction determined by a state value.
- `set_bit`: Sets a specific bit in a register.
- `wait_status_flag_ready`: Waits until a byte at a specific offset in a structure equals a fixed value and a flag byte is nonzero.
- `execute_linked_list_loop`: Repeatedly calls another routine in a loop until it returns zero.
- `initialize_data_loop`: Stores a longword into a memory location and enters a loop that repeatedly calls another routine.
- `advance_timed_event_sequence`: Decrements a delay counter and, when it reaches zero, advances through a table of timed entries to the first entry above the current value.
- `clear_register_byte`: Performs a logical OR on a register with zero.
- `select_copper_pointer_for_frame`: Reduces a position word modulo 80, then selects one of three copper-list addresses by a toggle and a state flag.
- `pack_sprite_control_bytes`: Writes four bytes into a record containing the low bytes of two values, their sum, and a flags byte based on high bits.
- `step_position_state_machine`: Writes a value into two copper pointer slots and moves a position by four units in a direction set by a state value.
- `init_record_defaults_after_wait`: Waits for a bit in a structure to clear, then initializes three longwords and two words in a record with specific values.
- `clear_data_byte`: Performs a logical OR operation on a data register with zero.
- `clear_data_register_byte`: Sets the lowest byte of a data register to zero.
- `tiny_ori_2`: Small routine of 1 instruction that merges 1 into d0.
- `process_table_entries`: Iterates through a table of values and, based on specific bitwise and magnitude checks, calls another routine and stores the result into a destination buffer.
- `transform_word_data`: Performs a series of bitwise manipulations and arithmetic adjustments on two words over three iterations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_unnamed_routine`: A routine consisting of twenty instructions that calls another routine.
