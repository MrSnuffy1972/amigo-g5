# Game 919 (turn n' burn (1990)(flair)[cr csl][t +2 pns]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00919). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 4 of 4 named (0 author symbols), 4 of 4 described (0 by rule, 4 by the local model) |
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
| Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS].adf:bootblock | 1 | 9 | 1 |
| logged disk load from Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS].adf boot-block load | 1 | 107 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |

## Cases handled

- turn n' burn (1990)(flair)[cr csl][t +2 pns]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS].adf: custom-format (non-DOS) disk, 1680 non-empty blocks: matched against track-loaded segments
- turn n' burn (1990)(flair)[cr csl][t +2 pns]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS].adf: the boot block reads 16384 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- turn n' burn (1990)(flair)[cr csl][t +2 pns]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS].adf: 1 logged loads cover 16384 bytes of the image
- turn n' burn (1990)(flair)[cr csl][t +2 pns]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS].adf: blocks 34-124 (bytes 17408-64000) outside the filesystem hold data, not code (opcode density 0.047, entropy 6.20)
- turn n' burn (1990)(flair)[cr csl][t +2 pns]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS].adf: blocks 127-129 (bytes 65024-66560) outside the filesystem hold data, not code (opcode density 0.020, entropy 2.47)
- turn n' burn (1990)(flair)[cr csl][t +2 pns]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS].adf: blocks 135-555 (bytes 69120-284672) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.87)
- turn n' burn (1990)(flair)[cr csl][t +2 pns]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS].adf: blocks 565-771 (bytes 289280-395264) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.39)
- turn n' burn (1990)(flair)[cr csl][t +2 pns]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS].adf: blocks 779-855 (bytes 398848-438272) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.14)
- turn n' burn (1990)(flair)[cr csl][t +2 pns]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS].adf: blocks 862-973 (bytes 441344-498688) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.07)
- turn n' burn (1990)(flair)[cr csl][t +2 pns]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS].adf: blocks 980-1089 (bytes 501760-558080) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.63)
- turn n' burn (1990)(flair)[cr csl][t +2 pns]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS].adf: blocks 1096-1190 (bytes 561152-609792) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.12)
- turn n' burn (1990)(flair)[cr csl][t +2 pns]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS].adf: blocks 1197-1272 (bytes 612864-651776) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.01)
- turn n' burn (1990)(flair)[cr csl][t +2 pns]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS].adf: blocks 1279-1363 (bytes 654848-698368) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.34)
- turn n' burn (1990)(flair)[cr csl][t +2 pns]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS].adf: blocks 1370-1457 (bytes 701440-746496) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.50)
- turn n' burn (1990)(flair)[cr csl][t +2 pns]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS].adf: blocks 1465-1554 (bytes 750080-796160) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.17)
- turn n' burn (1990)(flair)[cr csl][t +2 pns]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS].adf: blocks 1561-1636 (bytes 799232-838144) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.64)
- turn n' burn (1990)(flair)[cr csl][t +2 pns]/Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS].adf: blocks 1643-1759 (bytes 841216-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.26)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS].adf:bootblock

- `init_struct_and_call_exec`: Initializes a structure pointed to by a1 by writing the value 2 at offset 28, the value an address at offset 40, the value 25600 at offset 36, and the value 1024 at offset 44, then loads the exec library base into a6 and calls a routine there.

### logged disk load from Turn n' Burn (1990)(Flair)[cr CSL][t +2 PNS].adf boot-block load

- `save_byte_and_copy_block`: Reads a long from the stack into d0, saves its low byte into d7 and stores that byte into a structure field pointed to by a5, disables COP coprocessors, shifts d0 right by one bit with the carry preserved, returns, then copies a block of bytes backward through a2 using a loop and calls a routine.
- `shift_and_call_routine_loop`: Repeatedly shifts a value in d0 right by one bit while decrementing a counter in d1, calling a routine and rotating the result into d2 until the counter is exhausted, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_copper_and_disable_cop`: Compares the byte at absolute address 538d2 with the value 1 and, when they differ, writes the long at address 53884 to the copper list register, stores the long at address 53888 into a global variable, places a masked value into a structure field at offset 154, disables COP coprocessors, and stores the value 33777 into a structure field at offset 150.
