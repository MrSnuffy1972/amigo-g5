# Game 1369 (shadowlands (1992)(domark)[cr vf][m atx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01369). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (916064 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 23 raw-word lines |
| Functions named and described | pass, 16 of 16 named (0 author symbols), 16 of 16 described (1 by rule, 4 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 182 of 182 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 683 of 683 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 299843 |
| code | 2344 |
| data | 458546 |
| padding | 707 |
| unused | 154624 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Shadowlands (1992)(Domark)[cr VF][m ATX].adf:bootblock | 1 | 38 | 1 |
| logged disk load from Shadowlands (1992)(Domark)[cr VF][m ATX].adf boot-block load | 1 | 163 | 2 |
| logged disk load from Shadowlands (1992)(Domark)[cr VF][m ATX].adf boot-block load | 1 | 163 | 2 |
| logged disk load from Shadowlands (1992)(Domark)[cr VF][m ATX].adf boot-block load | 1 | 54 | 2 |
| logged disk load from Shadowlands (1992)(Domark)[cr VF][m ATX].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 202 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 63 | 2 |

## Cases handled

- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: custom-format (non-DOS) disk, 1447 non-empty blocks: matched against track-loaded segments
- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: the boot block reads 28672 bytes from disk offset 717824 to an address (found statically); recovered as a unit
- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: the boot block reads 16384 bytes from disk offset 701440 to an address (found statically); recovered as a unit
- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: the boot block reads 237568 bytes from disk offset 439296 to an address (found statically); recovered as a unit
- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: the boot block reads 16384 bytes from disk offset 856064 to an address (found statically); recovered as a unit
- runtime segment an address: 144 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 38 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: 4 logged loads cover 299008 bytes of the image
- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: blocks 2-3 (bytes 1024-2048) outside the filesystem hold data, not code (opcode density 0.023, entropy 3.81)
- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: blocks 6-9 (bytes 3072-5120) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.33)
- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: blocks 14-16 (bytes 7168-8704) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.79)
- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: blocks 20-22 (bytes 10240-11776) outside the filesystem hold data, not code (opcode density 0.016, entropy 6.74)
- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: blocks 26-29 (bytes 13312-15360) outside the filesystem hold data, not code (opcode density 0.013, entropy 6.80)
- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: blocks 32-34 (bytes 16384-17920) outside the filesystem hold data, not code (opcode density 0.010, entropy 6.77)
- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: blocks 38-40 (bytes 19456-20992) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.79)
- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: blocks 44-46 (bytes 22528-24064) outside the filesystem hold data, not code (opcode density 0.012, entropy 6.61)
- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: blocks 50-51 (bytes 25600-26624) outside the filesystem hold data, not code (opcode density 0.010, entropy 6.35)
- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: blocks 56-57 (bytes 28672-29696) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.34)
- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: blocks 62-62 (bytes 31744-32256) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.09)
- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: blocks 68-68 (bytes 34816-35328) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.10)
- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: blocks 74-74 (bytes 37888-38400) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.07)
- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: blocks 80-80 (bytes 40960-41472) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.07)
- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: blocks 86-86 (bytes 44032-44544) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.38)
- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: blocks 92-209 (bytes 47104-107520) outside the filesystem hold data, not code (opcode density 0.017, entropy 7.15)
- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: blocks 253-264 (bytes 129536-135680) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.43)
- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: blocks 270-857 (bytes 138240-439296) outside the filesystem hold data, not code (opcode density 0.006, entropy 4.98)
- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: blocks 1322-1369 (bytes 676864-701440) hold ASCII text
- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: blocks 1658-1671 (bytes 848896-856064) hold ASCII text
- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: blocks 1712-1747 (bytes 876544-894976) hold ASCII text
- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: blocks 1750-1753 (bytes 896000-898048) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.33)
- shadowlands (1992)(domark)[cr vf][m atx]/Shadowlands (1992)(Domark)[cr VF][m ATX].adf: blocks 1758-1759 (bytes 900096-901120) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.65)
- 1 packed track region(s), 8192 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Shadowlands (1992)(Domark)[cr VF][m ATX].adf:bootblock

- `issue_io_requests_then_jump`: Boot block entry that builds several exec DoIO request structures with memory addresses and lengths, and writes a clear-all mask to the DMA, interrupt and disk control registers.

### logged disk load from Shadowlands (1992)(Domark)[cr VF][m ATX].adf boot-block load

- `store_combined_byte_pair`: Reads a 32-bit value from memory just before the address register, then writes a 16-bit value built from the high byte of one register and the low byte of that value through a second pointer.
- `shift_bits_with_refill`: Shifts a bit buffer right once per iteration for a count taken from a second register, and calls an unnamed routine to refill the buffer whenever it becomes zero.

### logged disk load from Shadowlands (1992)(Domark)[cr VF][m ATX].adf boot-block load

- `store_combined_byte_pair_2`: Reads a 32-bit value from memory immediately before the address register, then stores a 16-bit value built from the high byte of one register and the low byte of that 32-bit value through the address register.
- `shift_bits_with_refill_2`: Loops a number of times taken from a register, shifting a bit buffer right once per iteration and reloading it from an unnamed routine whenever the buffer becomes empty.

### logged disk load from Shadowlands (1992)(Domark)[cr VF][m ATX].adf boot-block load

- `copy_block_and_jump_through_pointer`: Copies a block of longwords to a fixed low-memory address, writes two values to fixed locations, and then calls through a pointer in the copied block.
- `unpack_run_length_block`: Unpacks a run-length encoded stream into memory: a marker byte introduces a count and a value to repeat, and other bytes are copied through, until the input reaches a fixed end address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `forward_to_unnamed_routine`: Three-instruction routine in an unreached gap of code that calls one unnamed routine and returns.
- `dispatch_state_handler`: Reads a state value and clears it, then either returns early for states selected by a bit mask or calls a handler chosen from a table indexed by the state.
- `advance_cycling_table_index`: When a counter held in a global variable is zero, advances a two-bit index and loads the word at that index from a table.
- `update_audio_channels_per_tick`: Advances a counter in a global variable and writes to the interrupt request register; when the counter is zero it writes a fixed address and length pair to each of the four audio channel registers.
- `read_joystick_and_mouse_input`: Reads the mouse and joystick movement registers to update two position globals clamped to 0 to 319 and 0 to 199, then samples the fire button, disk status and right and middle button bits.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `split_pairs_into_two_buffers`: Copies sixteen pairs of longwords from a block referenced through an address register into two fixed buffers, sending the first of each pair to one buffer and the second to the other.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `search_coordinate_list`: Iterates through a list of four-word structures to find an entry where two specific values fall within a defined range.
- `update_status_flags`: Checks a global byte and specific memory offsets to conditionally perform a bitwise XOR operation on a value in a table and update a global variable.
