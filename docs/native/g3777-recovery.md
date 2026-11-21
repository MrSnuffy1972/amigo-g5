# Game 3777 (Garfield - Winter's Tail): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03777). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 1 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 4 of 4 named (0 author symbols), 4 of 4 described (1 by rule, 3 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 32 of 32 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 49 of 49 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 5431 |
| code | 190 |
| data | 856075 |
| unused | 39424 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Garfield - Winter's Tail.adf:bootblock | 1 | 14 | 1 |
| logged disk load from Garfield - Winter's Tail.adf boot-block load | 1 | 35 | 3 |

## Cases handled

- Garfield - Winter's Tail/Garfield - Winter's Tail.adf: custom-format (non-DOS) disk, 1681 non-empty blocks: matched against track-loaded segments
- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 75D4) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- Garfield - Winter's Tail/Garfield - Winter's Tail.adf: the boot block reads 5632 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- Garfield - Winter's Tail/Garfield - Winter's Tail.adf: 1 logged loads cover 5632 bytes of the image
- Garfield - Winter's Tail/Garfield - Winter's Tail.adf: blocks 22-112 (bytes 11264-57856) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.52)
- Garfield - Winter's Tail/Garfield - Winter's Tail.adf: blocks 115-139 (bytes 58880-71680) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.38)
- Garfield - Winter's Tail/Garfield - Winter's Tail.adf: blocks 143-712 (bytes 73216-365056) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.99)
- Garfield - Winter's Tail/Garfield - Winter's Tail.adf: blocks 715-1031 (bytes 366080-528384) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.58)
- Garfield - Winter's Tail/Garfield - Winter's Tail.adf: blocks 1034-1227 (bytes 529408-628736) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.33)
- Garfield - Winter's Tail/Garfield - Winter's Tail.adf: blocks 1231-1442 (bytes 630272-738816) outside the filesystem hold data, not code (opcode density 0.016, entropy 4.44)
- Garfield - Winter's Tail/Garfield - Winter's Tail.adf: blocks 1446-1471 (bytes 740352-753664) outside the filesystem hold data, not code (opcode density 0.007, entropy 2.83)
- Garfield - Winter's Tail/Garfield - Winter's Tail.adf: blocks 1473-1484 (bytes 754176-760320) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.90)
- Garfield - Winter's Tail/Garfield - Winter's Tail.adf: blocks 1495-1506 (bytes 765440-771584) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.02)
- Garfield - Winter's Tail/Garfield - Winter's Tail.adf: blocks 1517-1528 (bytes 776704-782848) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.27)
- Garfield - Winter's Tail/Garfield - Winter's Tail.adf: blocks 1539-1550 (bytes 787968-794112) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.08)
- Garfield - Winter's Tail/Garfield - Winter's Tail.adf: blocks 1562-1572 (bytes 799744-805376) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.54)
- Garfield - Winter's Tail/Garfield - Winter's Tail.adf: blocks 1584-1759 (bytes 811008-901120) outside the filesystem hold data, not code (opcode density 0.046, entropy 4.74)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Garfield - Winter's Tail.adf:bootblock

- `initialize_video_registers`: Sets multiple video color registers to specific values and performs system calls to configure display hardware.

### logged disk load from Garfield - Winter's Tail.adf boot-block load

- `xor_memory_loop`: Executes a complex loop that repeatedly XORs memory values with a global variable, performs bitwise operations on pointers, and manipulates various memory locations including a global variable and several pointer targets.
- `bitwise_memory_operations`: Performs a series of bitwise operations on memory locations using registers and pointer arithmetic.
- `frag_add_move_roxl_bne_bcs`: Short code fragment that adds the memory at a5 to d1, then copies the memory at a4 (advancing a4) to the memory at a0 (advancing a0), then rotates d4 left through the extend flag by 2, then branches if the last result was not zero (2 times in a row), then branches if the carry flag is set.
