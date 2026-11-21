# Game 4598 (operation wolf (1988)(ocean)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04598). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802320 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 2 of 2 named (0 author symbols), 2 of 2 described (1 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2 of 2 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 31 of 31 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 154 |
| data | 1787830 |
| unused | 14336 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Operation Wolf (1988)(Ocean)(Disk 1 of 2).adf:bootblock | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |

## Cases handled

- operation wolf (1988)(ocean)(disk 1 of 2)/Operation Wolf (1988)(Ocean)(Disk 1 of 2).adf: custom-format (non-DOS) disk, 1733 non-empty blocks: matched against track-loaded segments
- operation wolf (1988)(ocean)(disk 2 of 2)/Operation Wolf (1988)(Ocean)(Disk 2 of 2).adf: custom-format (non-DOS) disk, 1755 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- operation wolf (1988)(ocean)(disk 1 of 2)/Operation Wolf (1988)(Ocean)(Disk 1 of 2).adf: blocks 2-10 (bytes 1024-5632) outside the filesystem hold data, not code (opcode density 0.028, entropy 6.92)
- operation wolf (1988)(ocean)(disk 1 of 2)/Operation Wolf (1988)(Ocean)(Disk 1 of 2).adf: blocks 22-156 (bytes 11264-80384) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.75)
- operation wolf (1988)(ocean)(disk 1 of 2)/Operation Wolf (1988)(Ocean)(Disk 1 of 2).adf: blocks 159-164 (bytes 81408-84480) outside the filesystem hold data, not code (opcode density 0.007, entropy 1.83)
- operation wolf (1988)(ocean)(disk 1 of 2)/Operation Wolf (1988)(Ocean)(Disk 1 of 2).adf: blocks 167-524 (bytes 85504-268800) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.02)
- operation wolf (1988)(ocean)(disk 1 of 2)/Operation Wolf (1988)(Ocean)(Disk 1 of 2).adf: blocks 526-1188 (bytes 269312-608768) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.73)
- operation wolf (1988)(ocean)(disk 1 of 2)/Operation Wolf (1988)(Ocean)(Disk 1 of 2).adf: blocks 1197-1496 (bytes 612864-766464) outside the filesystem hold data, not code (opcode density 0.048, entropy 6.15)
- operation wolf (1988)(ocean)(disk 1 of 2)/Operation Wolf (1988)(Ocean)(Disk 1 of 2).adf: blocks 1498-1759 (bytes 766976-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.43)
- operation wolf (1988)(ocean)(disk 2 of 2)/Operation Wolf (1988)(Ocean)(Disk 2 of 2).adf: blocks 2-1144 (bytes 1024-586240) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.34)
- operation wolf (1988)(ocean)(disk 2 of 2)/Operation Wolf (1988)(Ocean)(Disk 2 of 2).adf: blocks 1146-1579 (bytes 586752-808960) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.52)
- operation wolf (1988)(ocean)(disk 2 of 2)/Operation Wolf (1988)(Ocean)(Disk 2 of 2).adf: blocks 1582-1759 (bytes 809984-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.35)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Operation Wolf (1988)(Ocean)(Disk 1 of 2).adf:bootblock

- `allocate_and_io`: Allocates memory and performs an I/O operation using a global variable, then checks the result and repeats until successful.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_cmpi_blt_movea_adda_move_bsr`: Short code fragment that compares a fixed memory address with 170, then branches if the comparison was less, then copies a fixed memory address to a1, then adds 54412 to a1, then copies 32000 to d7, then copies a fixed memory address to a0, then calls another routine, then copies a fixed memory address to a fixed memory address, then copies an address to a fixed memory address, then calls another routine.
