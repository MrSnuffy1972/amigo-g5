# Game 378 (Saint Dragon): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00378). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 5 of 5 named (0 author symbols), 5 of 5 described (1 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 139 of 139 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 247 of 247 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 232 |
| code | 632 |
| data | 507968 |
| padding | 1120 |
| unused | 391168 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Saint Dragon.adf:SKR_VAL | 1 | 154 | 1 |
| Saint Dragon.adf:SD | 2 | 83 | 3 |
| Saint Dragon.adf:bootblock | 1 | 10 | 1 |

## Cases handled

- Saint Dragon/Saint Dragon.adf: blocks 2-2 (bytes 1024-1536) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.72)
- Saint Dragon/Saint Dragon.adf: blocks 4-849 (bytes 2048-435200) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.65)
- Saint Dragon/Saint Dragon.adf:SKR_VAL: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Saint Dragon.adf:SKR_VAL

- `copy_byte_block`: Copies a block of bytes from one memory area to another, then reads a byte from a table and copies a variable number of bytes from it to a destination buffer depending on the byte's value, writing a counter to a hardware register along the way.

### Saint Dragon.adf:SD

- `intro_bytekiller_depack`: Cracktro start: ByteKiller depacker that decodes the packed stream after it backwards into 50000 hex onwards, showing progress on COLOR00, checks the checksum and jumps to the unpacked cracktro at 50000 hex.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Saint Dragon.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
