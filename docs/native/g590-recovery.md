# Game 590 (Rainbow Islands): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00590). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (909120 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 21 of 21 named (0 author symbols), 21 of 21 described (3 by rule, 18 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 48 of 48 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 94208 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 321 of 321 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 16241 |
| code | 936 |
| data | 690211 |
| padding | 4 |
| unused | 201728 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Rainbow Islands.adf:bootblock | 1 | 17 | 1 |
| logged disk load from Rainbow Islands.adf boot-block load | 1 | 34 | 1 |
| logged disk load from Rainbow Islands.adf boot-block load | 1 | 205 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 40 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 3 |
| Rainbow Islands.adf bytes 34816-88576 (code never loaded by a reference run) | 1 | 0 | 0 |
| Rainbow Islands.adf bytes 91648-120832 (code never loaded by a reference run) | 1 | 0 | 0 |
| Rainbow Islands.adf bytes 256000-267264 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- Rainbow Islands/Rainbow Islands.adf: custom-format (non-DOS) disk, 1359 non-empty blocks: matched against track-loaded segments
- Rainbow Islands/Rainbow Islands.adf: the boot block reads 11264 bytes from disk offset 889856 to an address (found statically); recovered as a unit
- Rainbow Islands/Rainbow Islands.adf: the boot block reads 5632 bytes from disk offset 867328 to an address (found statically); recovered as a unit
- runtime segment an address: 30 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 17 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Rainbow Islands/Rainbow Islands.adf: bytes 34816-88576 look like code no run loaded: recovered position-independently (load address unknown)
- Rainbow Islands/Rainbow Islands.adf: bytes 91648-120832 look like code no run loaded: recovered position-independently (load address unknown)
- Rainbow Islands/Rainbow Islands.adf: bytes 256000-267264 look like code no run loaded: recovered position-independently (load address unknown)
- Rainbow Islands/Rainbow Islands.adf: 5 logged loads cover 111104 bytes of the image
- Rainbow Islands/Rainbow Islands.adf: 1 track-loaded blocks found in recovered runtime segments and classified through them
- Rainbow Islands/Rainbow Islands.adf: blocks 2-2 (bytes 1024-1536) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.65)
- Rainbow Islands/Rainbow Islands.adf: blocks 22-29 (bytes 11264-15360) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.12)
- Rainbow Islands/Rainbow Islands.adf: blocks 32-39 (bytes 16384-20480) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.95)
- Rainbow Islands/Rainbow Islands.adf: blocks 65-66 (bytes 33280-34304) outside the filesystem hold data, not code (opcode density 0.002, entropy 1.29)
- Rainbow Islands/Rainbow Islands.adf: blocks 242-498 (bytes 123904-255488) outside the filesystem hold data, not code (opcode density 0.013, entropy 4.45)
- Rainbow Islands/Rainbow Islands.adf: blocks 523-523 (bytes 267776-268288) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.61)
- Rainbow Islands/Rainbow Islands.adf: blocks 525-525 (bytes 268800-269312) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.48)
- Rainbow Islands/Rainbow Islands.adf: blocks 527-551 (bytes 269824-282624) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.26)
- Rainbow Islands/Rainbow Islands.adf: blocks 553-727 (bytes 283136-372736) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.76)
- Rainbow Islands/Rainbow Islands.adf: blocks 729-823 (bytes 373248-421888) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.17)
- Rainbow Islands/Rainbow Islands.adf: blocks 825-925 (bytes 422400-474112) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.53)
- Rainbow Islands/Rainbow Islands.adf: blocks 927-1037 (bytes 474624-531456) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.90)
- Rainbow Islands/Rainbow Islands.adf: blocks 1039-1149 (bytes 531968-588800) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.03)
- Rainbow Islands/Rainbow Islands.adf: blocks 1151-1273 (bytes 589312-652288) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.38)
- Rainbow Islands/Rainbow Islands.adf: blocks 1275-1336 (bytes 652800-684544) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.86)
- Rainbow Islands/Rainbow Islands.adf: blocks 1347-1347 (bytes 689664-690176) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.79)
- Rainbow Islands/Rainbow Islands.adf: blocks 1349-1400 (bytes 690688-717312) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.33)
- Rainbow Islands/Rainbow Islands.adf: blocks 1705-1710 (bytes 872960-876032) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.80)
- Rainbow Islands/Rainbow Islands.adf: blocks 1727-1732 (bytes 884224-887296) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.80)
- 3 never-loaded region(s), 94208 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Rainbow Islands.adf:bootblock

- `initialize_boot_io`: Sets up two IO requests in a structure in a1 with different memory ranges and lengths, calls a routine via a pointer in a6 twice, then clears two global bytes and jumps to a location in a global variable.

### logged disk load from Rainbow Islands.adf boot-block load

- `scale_palette_per_frame`: Waits for and acknowledges the vertical blank request bit, then scales a table of 16 color values by a level that rises each frame and writes them into the palette buffer.

### logged disk load from Rainbow Islands.adf boot-block load

- `copy_block_with_alignment_fixups`: Copies a block of data between buffers, adjusting the source and destination pointers to even addresses before copying runs of longwords.
- `initialize_huffman_decoding_tables`: Reads a 16-bit value from a source buffer, performs bit manipulation on it, and calls three separate routines to populate three different segments of a table in a2 at offsets 0, 128, and 256 respectively, before decrementing a counter and branching.
- `copy_and_extract_bitfield_data`: Performs two memory copy operations using lengths determined by a search routine, then extracts a bitfield from a source buffer into a destination buffer using a bit-shift value stored in a register.
- `match_bitmasked_pair_and_extract_bits`: Scans word pairs at a0 until the masked first word equals the second, then reads two byte fields and returns a bit value built from d6 shifted by the field counts, calling the refill routine when d7 runs low.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 255 into d0, then loads 16 into d1.
- `mask_d0_with_d6_and_refill_bits`: Masks d0 with d6, subtracts d1 from d7 and, if the bit budget went negative, calls the refill routine before shifting d6 right by d1 and returning the masked value.
- `refill_bit_buffer_from_a3_stream`: Adds d1 back into d7, shifts d6 right by d7, swaps d6, advances a3 by four and loads two bytes backwards into d6 with a byte swap, then resets d7 to sixteen minus d1.
- `read_four_bytes_into_d0`: Loads four bytes from the address in a0 into the low bytes of d0, advancing a0 by four, then returns.
- `build_huffman_decode_tables`: Reads a code-length table via a helper, then fills a0 with per-symbol masks, bit patterns, lengths and offsets for decoding, returning after all lengths are processed.
- `build_jump_table`: Copies ten 16-bit values from a location relative to the program counter onto the stack, then jumps to a routine using a pointer from the stack.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_indirect_and_restore`: Calls a routine using an address calculated from a4 and d7, then restores a4, a6, a2, and a3 from the stack before returning.
- `increment_counter_and_call_handler`: Increments a global counter, then calls a routine whose address is read from a pointer cell plus a 16-bit offset stored 38 bytes into the structure it points to.
- `dispatch_interrupt_handlers`: Reads bits from a hardware register, writes the result to a specific offset in that register, and then calls one of three routines based on the value of the processed bits.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_indirect_call`: Calls a subroutine using an address calculated by adding two values together, then restores a saved pointer and returns.
- `store_word_triple`: Copies three consecutive 16-bit values from a source pointer into three consecutive 16-bit fields of a record at offset 30.
- `follow_linked_node_entries`: Follows the first link from a record and, when that node has a successor, calls the entry routine stored at offset 16 of the node.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_and_jump_to_address`: Sets bit 5 of d3 and jumps to an address stored in a register.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
