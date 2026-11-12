# Game 920 (turn n' burn (1990)(flair)[cr csl][t +2 pns][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00920). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 4 of 4 named (0 author symbols), 4 of 4 described (0 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2 of 2 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 126 of 126 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 15866 |
| code | 386 |
| data | 844996 |
| unused | 39936 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS][a].adf:bootblock | 1 | 9 | 1 |
| logged disk load from Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS][a].adf boot-block load | 1 | 107 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |

## Cases handled

- turn n' burn (1990)(flair)[cr csl][t +2 pns][a]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS][a].adf: custom-format (non-DOS) disk, 1680 non-empty blocks: matched against track-loaded segments
- turn n' burn (1990)(flair)[cr csl][t +2 pns][a]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS][a].adf: the boot block reads 16384 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- turn n' burn (1990)(flair)[cr csl][t +2 pns][a]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS][a].adf: 1 logged loads cover 16384 bytes of the image
- turn n' burn (1990)(flair)[cr csl][t +2 pns][a]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS][a].adf: blocks 34-124 (bytes 17408-64000) outside the filesystem hold data, not code (opcode density 0.047, entropy 6.20)
- turn n' burn (1990)(flair)[cr csl][t +2 pns][a]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS][a].adf: blocks 127-129 (bytes 65024-66560) outside the filesystem hold data, not code (opcode density 0.020, entropy 2.47)
- turn n' burn (1990)(flair)[cr csl][t +2 pns][a]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS][a].adf: blocks 135-555 (bytes 69120-284672) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.87)
- turn n' burn (1990)(flair)[cr csl][t +2 pns][a]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS][a].adf: blocks 565-771 (bytes 289280-395264) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.39)
- turn n' burn (1990)(flair)[cr csl][t +2 pns][a]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS][a].adf: blocks 779-855 (bytes 398848-438272) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.14)
- turn n' burn (1990)(flair)[cr csl][t +2 pns][a]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS][a].adf: blocks 862-973 (bytes 441344-498688) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.07)
- turn n' burn (1990)(flair)[cr csl][t +2 pns][a]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS][a].adf: blocks 980-1089 (bytes 501760-558080) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.63)
- turn n' burn (1990)(flair)[cr csl][t +2 pns][a]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS][a].adf: blocks 1096-1190 (bytes 561152-609792) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.12)
- turn n' burn (1990)(flair)[cr csl][t +2 pns][a]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS][a].adf: blocks 1197-1272 (bytes 612864-651776) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.01)
- turn n' burn (1990)(flair)[cr csl][t +2 pns][a]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS][a].adf: blocks 1279-1363 (bytes 654848-698368) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.34)
- turn n' burn (1990)(flair)[cr csl][t +2 pns][a]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS][a].adf: blocks 1370-1457 (bytes 701440-746496) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.50)
- turn n' burn (1990)(flair)[cr csl][t +2 pns][a]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS][a].adf: blocks 1465-1554 (bytes 750080-796160) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.17)
- turn n' burn (1990)(flair)[cr csl][t +2 pns][a]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS][a].adf: blocks 1561-1636 (bytes 799232-838144) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.64)
- turn n' burn (1990)(flair)[cr csl][t +2 pns][a]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS][a].adf: blocks 1643-1759 (bytes 841216-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.26)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS][a].adf:bootblock

- `issue_boot_disk_io_request`: Boot block entry point, called by the Kickstart strap, that issues one request through the Exec DoIO library routine.

### logged disk load from Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS][a].adf boot-block load

- `shift_long_and_store_word`: Reads the longword four bytes before the address register, writes a word built from its low byte and the high byte of a data register through a pointer, and returns the value shifted right one bit with the top bit set.
- `shift_bits_through_refill_loop`: Shifts a bit register right one place per step for a count taken from its argument, calling a refill routine whenever the register empties.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_display_and_coprocessor_state`: Checks a global byte for a specific value and, if matched, updates the bitplane pointer in the display hardware, modifies a word in a global structure via a6 offset 154, writes a byte to a global variable, and sets a word in a global structure via a6 offset 150.
