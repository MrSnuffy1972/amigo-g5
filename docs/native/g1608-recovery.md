# Game 1608 (rainbow islands (1987)(ocean)[cr scx][t +3 scx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01608). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (909120 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 14 of 14 named (0 author symbols), 14 of 14 described (1 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 48 of 48 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 94208 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 204 of 204 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 33307 |
| code | 672 |
| data | 684161 |
| padding | 4 |
| unused | 190976 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf:bootblock | 1 | 15 | 1 |
| logged disk load from Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf boot-block load | 1 | 34 | 1 |
| logged disk load from Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf boot-block load | 1 | 90 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 40 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 3 |
| Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf bytes 34816-88576 (code never loaded by a reference run) | 1 | 0 | 0 |
| Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf bytes 91648-120832 (code never loaded by a reference run) | 1 | 0 | 0 |
| Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf bytes 256000-267264 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: custom-format (non-DOS) disk, 1359 non-empty blocks: matched against track-loaded segments
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: the boot block reads 11264 bytes from disk offset 889856 to an address (found statically); recovered as a unit
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: the boot block reads 22528 bytes from disk offset 867328 to an address (found statically); recovered as a unit
- runtime segment an address: 30 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 17 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: bytes 34816-88576 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: bytes 91648-120832 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: bytes 256000-267264 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: 5 logged loads cover 128000 bytes of the image
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: 1 track-loaded blocks found in recovered runtime segments and classified through them
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 2-2 (bytes 1024-1536) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.65)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 22-29 (bytes 11264-15360) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.12)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 32-39 (bytes 16384-20480) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.95)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 65-66 (bytes 33280-34304) outside the filesystem hold data, not code (opcode density 0.002, entropy 1.29)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 242-498 (bytes 123904-255488) outside the filesystem hold data, not code (opcode density 0.013, entropy 4.45)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 523-523 (bytes 267776-268288) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.61)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 525-525 (bytes 268800-269312) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.48)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 527-551 (bytes 269824-282624) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.26)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 553-727 (bytes 283136-372736) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.76)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 729-823 (bytes 373248-421888) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.17)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 825-925 (bytes 422400-474112) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.53)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 927-1037 (bytes 474624-531456) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.90)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 1039-1149 (bytes 531968-588800) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.03)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 1151-1273 (bytes 589312-652288) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.38)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 1275-1336 (bytes 652800-684544) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.86)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 1347-1347 (bytes 689664-690176) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.79)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 1349-1400 (bytes 690688-717312) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.33)
- 3 never-loaded region(s), 94208 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf:bootblock

- `load_boot_data_then_jump`: Disables all interrupts and DMA channels, issues two exec DoIO requests, clears a 16-long global area, traps once, copies 256 longwords into low memory and calls a routine through a pointer.

### logged disk load from Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf boot-block load

- `scale_palette_per_frame`: Waits for and acknowledges the vertical blank request bit, then scales a table of 16 color values by a level that rises each frame and writes them into the palette buffer.

### logged disk load from Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf boot-block load

- `write_decrementing_global_until_wrap`: Writes a decrementing 16-bit value into a global cell in a loop that ends when the counter wraps past zero, then returns minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_indirect_call`: Calculates an address by adding two values and jumps to that location.
- `increment_counter_and_call_handler`: Increments a global counter, then calls a routine whose address is read from a pointer cell plus a 16-bit offset stored 38 bytes into the structure it points to.
- `dispatch_pending_interrupt_levels`: Reads the interrupt enable and request registers, masks the result to interrupt bits four to six, writes the masked bits to the interrupt request register, and calls up to three level handlers through function cells according to those bits.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `perform_indirect_jump`: Executes a subroutine call using a destination address derived from the sum of two registers.
- `store_word_triple`: Copies three consecutive 16-bit values from a source pointer into three consecutive 16-bit fields of a structure at fixed offsets, and returns minus one.
- `execute_callback_chain`: Walks a singly linked list starting from a pointer held in a structure, calling the routine at offset 16 in each node until a node's next pointer is zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_once_then_increment_counter`: Calls a routine through a table entry when a 16-bit counter in the structure is zero, then increments the counter.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `call_structure_callback`: Calls the routine whose address is stored 20 bytes into the structure pointed to by the base register, then returns.
