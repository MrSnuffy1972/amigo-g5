# Game 560 (Race Drivin'): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00560). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802240 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 8 of 8 named (0 author symbols), 8 of 8 described (0 by rule, 6 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 9728 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 284 of 483 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 10239 |
| code | 1438 |
| data | 591457 |
| padding | 2 |
| unused | 1199104 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Race Drivin' (1990)(Domark)(Disk 1 of 2).adf:bootblock | 1 | 22 | 1 |
| Race Drivin' (1990)(Domark)(Disk 2 of 2).adf:bootblock | 1 | 250 | 1 |
| logged disk load from Race Drivin' (1990)(Domark)(Disk 1 of 2).adf boot-block load | 1 | 18 | 2 |
| logged disk load from Race Drivin' (1990)(Domark)(Disk 2 of 2).adf boot-block load | 1 | 94 | 2 |
| logged disk load from Race Drivin' (1990)(Domark)(Disk 2 of 2).adf boot-block load | 1 | 99 | 2 |
| Race Drivin' (1990)(Domark)(Disk 1 of 2).adf bytes 23552-33280 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: custom-format (non-DOS) disk, 1164 non-empty blocks: matched against track-loaded segments
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: the boot block reads 8192 bytes from disk offset 15360 to an address (found statically); recovered as a unit
- race drivin' (1990)(domark)(disk 2 of 2)/Race Drivin' (1990)(Domark)(Disk 2 of 2).adf: the boot block reads 4096 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- race drivin' (1990)(domark)(disk 2 of 2)/Race Drivin' (1990)(Domark)(Disk 2 of 2).adf: the boot block reads 124416 bytes from disk offset 11264 to an address (found statically); recovered as a unit
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: bytes 23552-33280 look like code no run loaded: recovered position-independently (load address unknown)
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: 2 logged loads cover 17920 bytes of the image
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 4-4 (bytes 2048-2560) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.28)
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 22-22 (bytes 11264-11776) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 68-1168 (bytes 34816-598528) hold 9 packed streams (Ice: Pack-Ice v2.0 - v2.20, SCO Compress LZH) found by scanning, 99% of the bytes
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 1188-1188 (bytes 608256-608768) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 1210-1210 (bytes 619520-620032) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 1232-1232 (bytes 630784-631296) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 1254-1254 (bytes 642048-642560) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 1276-1276 (bytes 653312-653824) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 1298-1298 (bytes 664576-665088) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 1320-1320 (bytes 675840-676352) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 1342-1342 (bytes 687104-687616) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 1364-1364 (bytes 698368-698880) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 1386-1386 (bytes 709632-710144) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 1408-1408 (bytes 720896-721408) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 1430-1430 (bytes 732160-732672) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 1452-1452 (bytes 743424-743936) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 1474-1474 (bytes 754688-755200) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 1496-1496 (bytes 765952-766464) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 1518-1518 (bytes 777216-777728) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 1540-1540 (bytes 788480-788992) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 1562-1562 (bytes 799744-800256) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 1584-1584 (bytes 811008-811520) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 1606-1606 (bytes 822272-822784) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 1628-1628 (bytes 833536-834048) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 1650-1650 (bytes 844800-845312) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 1672-1672 (bytes 856064-856576) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 1694-1694 (bytes 867328-867840) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 1716-1716 (bytes 878592-879104) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- race drivin' (1990)(domark)(disk 1 of 2)/Race Drivin' (1990)(Domark)(Disk 1 of 2).adf: blocks 1738-1738 (bytes 889856-890368) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- race drivin' (1990)(domark)(disk 2 of 2)/Race Drivin' (1990)(Domark)(Disk 2 of 2).adf: 2 logged loads cover 4096 bytes of the image
- 1 never-loaded region(s), 9728 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Race Drivin' (1990)(Domark)(Disk 1 of 2).adf:bootblock

- `setup_memory_and_loop`: Allocates memory through the exec library and, when the allocation succeeds, fills a structure with device parameters, calls the library again, triggers a system trap, and then writes incrementing values to a color register in an endless loop.

### Race Drivin' (1990)(Domark)(Disk 2 of 2).adf:bootblock

- `init_device_and_dispatch`: Initializes a device structure, calls a routine at a computed address, then repeatedly calls another routine through a table until a status byte becomes zero before invoking two more routines and jumping to a recovered address.

### logged disk load from Race Drivin' (1990)(Domark)(Disk 1 of 2).adf boot-block load

- `copy_bytes_and_jump`: Disables interrupts via the system register, compares two values and returns otherwise, then copies thirty bytes from a source to a destination in a loop before jumping to the address held in register a0.
- `copy_words_and_jump`: Loads the decimal value 1435 into a register, copies five words from the source pointed to by a0 to the destination pointed to by a1 using pre-increment, decrements the counter in a loop, and finally jumps to the address in register d1.

### logged disk load from Race Drivin' (1990)(Domark)(Disk 2 of 2).adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from Race Drivin' (1990)(Domark)(Disk 2 of 2).adf boot-block load

- `rotate_and_copy_data`: Reads a longword from a decreasing source, stores its low byte as a word, disables interrupts, rotates the value, and returns; otherwise it copies data through a table in a loop and branches based on pointer comparison.
- `recursive_bit_processing`: Loops while decrementing a counter, and when a shifted value reaches zero it calls another routine and updates the counter, otherwise it returns; it then branches on a compared value to call itself with different arguments.
