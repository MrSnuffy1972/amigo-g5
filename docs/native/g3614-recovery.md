# Game 3614 (federation quest (19xx)(-)(beta)[cr black monks]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03614). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1804816 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 14 of 14 named (0 author symbols), 14 of 14 described (5 by rule, 8 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 133 of 133 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 91136 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 325 of 325 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 12585 |
| code | 1238 |
| data | 863755 |
| padding | 6 |
| unused | 927232 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Federation Quest (19xx)(-)(beta)[cr Black Monks](Disk 1 of 2).adf:bootblock | 1 | 6 | 1 |
| logged disk load from Federation Quest (19xx)(-)(beta)[cr Black Monks](Disk 1 of 2).adf boot-block load | 1 | 107 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 204 | 11 |
| Federation Quest (19xx)(-)(beta)[cr Black Monks](Disk 1 of 2).adf bytes 1024-26112 (code never loaded by a reference run) | 1 | 0 | 0 |
| Federation Quest (19xx)(-)(beta)[cr Black Monks](Disk 1 of 2).adf bytes 27648-93696 (code never loaded by a reference run) | 1 | 8 | 0 |

## Cases handled

- federation quest (19xx)(-)(beta)[cr black monks](disk 1 of 2)/Federation Quest (19xx)(-)(beta)[cr Black Monks](Disk 1 of 2).adf: custom-format (non-DOS) disk, 1692 non-empty blocks: matched against track-loaded segments
- federation quest (19xx)(-)(beta)[cr black monks](disk 1 of 2)/Federation Quest (19xx)(-)(beta)[cr Black Monks](Disk 1 of 2).adf: the boot block reads 11264 bytes from disk offset 867328 to an address (found statically); recovered as a unit
- runtime segment an address: 133 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- federation quest (19xx)(-)(beta)[cr black monks](disk 1 of 2)/Federation Quest (19xx)(-)(beta)[cr Black Monks](Disk 1 of 2).adf: bytes 1024-26112 look like code no run loaded: recovered position-independently (load address unknown)
- federation quest (19xx)(-)(beta)[cr black monks](disk 1 of 2)/Federation Quest (19xx)(-)(beta)[cr Black Monks](Disk 1 of 2).adf: bytes 27648-93696 look like code no run loaded: recovered position-independently (load address unknown)
- federation quest (19xx)(-)(beta)[cr black monks](disk 1 of 2)/Federation Quest (19xx)(-)(beta)[cr Black Monks](Disk 1 of 2).adf: 3 logged loads cover 102400 bytes of the image
- federation quest (19xx)(-)(beta)[cr black monks](disk 1 of 2)/Federation Quest (19xx)(-)(beta)[cr Black Monks](Disk 1 of 2).adf: blocks 189-729 (bytes 96768-373760) outside the filesystem hold data, not code (opcode density 0.009, entropy 7.02)
- federation quest (19xx)(-)(beta)[cr black monks](disk 1 of 2)/Federation Quest (19xx)(-)(beta)[cr Black Monks](Disk 1 of 2).adf: blocks 731-761 (bytes 374272-390144) outside the filesystem hold data, not code (opcode density 0.030, entropy 6.96)
- federation quest (19xx)(-)(beta)[cr black monks](disk 1 of 2)/Federation Quest (19xx)(-)(beta)[cr Black Monks](Disk 1 of 2).adf: blocks 770-865 (bytes 394240-443392) outside the filesystem hold data, not code (opcode density 0.007, entropy 7.10)
- federation quest (19xx)(-)(beta)[cr black monks](disk 1 of 2)/Federation Quest (19xx)(-)(beta)[cr Black Monks](Disk 1 of 2).adf: blocks 869-1304 (bytes 444928-668160) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.87)
- federation quest (19xx)(-)(beta)[cr black monks](disk 1 of 2)/Federation Quest (19xx)(-)(beta)[cr Black Monks](Disk 1 of 2).adf: blocks 1320-1336 (bytes 675840-684544) outside the filesystem hold data, not code (opcode density 0.008, entropy 7.09)
- federation quest (19xx)(-)(beta)[cr black monks](disk 1 of 2)/Federation Quest (19xx)(-)(beta)[cr Black Monks](Disk 1 of 2).adf: blocks 1342-1375 (bytes 687104-704512) outside the filesystem hold data, not code (opcode density 0.011, entropy 4.57)
- federation quest (19xx)(-)(beta)[cr black monks](disk 1 of 2)/Federation Quest (19xx)(-)(beta)[cr Black Monks](Disk 1 of 2).adf: blocks 1485-1493 (bytes 760320-764928) outside the filesystem hold data, not code (opcode density 0.009, entropy 7.19)
- federation quest (19xx)(-)(beta)[cr black monks](disk 1 of 2)/Federation Quest (19xx)(-)(beta)[cr Black Monks](Disk 1 of 2).adf: blocks 1496-1693 (bytes 765952-867328) outside the filesystem hold data, not code (opcode density 0.013, entropy 6.72)
- federation quest (19xx)(-)(beta)[cr black monks](disk 1 of 2)/Federation Quest (19xx)(-)(beta)[cr Black Monks](Disk 1 of 2).adf: blocks 1716-1759 (bytes 878592-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.35)
- federation quest (19xx)(-)(beta)[cr black monks](disk 2 of 2)/Federation Quest (19xx)(-)(beta)[cr Black Monks](Disk 2 of 2).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- 2 never-loaded region(s), 91136 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 49664 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Federation Quest (19xx)(-)(beta)[cr Black Monks](Disk 1 of 2).adf:bootblock

- `frag_move_jsr_jmp`: Short code fragment that copies 2 to the memory at an offset from a1, then copies an address to the memory at an offset from a1, then copies an address to the memory at an offset from a1, then copies 11264 to the memory at an offset from a1, then calls another routine, then jumps to another point in the code.

### logged disk load from Federation Quest (19xx)(-)(beta)[cr Black Monks](Disk 1 of 2).adf boot-block load

- `process_and_shift_data`: Processes data by moving values from memory to registers, performs bitwise operations, and returns a shifted result with specific bit patterns.
- `process_bitfield_data`: Processes bitfield data by shifting bits, calling other routines, and managing loop counters to handle sequential operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_dma_and_blitter`: Sets up DMA and blitter parameters by configuring hardware registers, calling multiple setup routines, and managing display-related data through specific memory accesses and register operations.
- `build_repeated_longword_pattern`: Writes one changing longword followed by forty-two copies of a fixed longword for 200 groups, advancing the destination and incrementing the changing value each group.
- `clear_memory_blocks`: Clears memory blocks at three different addresses with varying sizes and data types, using loop counters to iterate through each block and zeroing out the memory locations sequentially.
- `frag_move_lea_swap_rts`: Short code fragment that copies a fixed memory address to d0, then copies a fixed memory address to a fixed memory address, then copies d0 to a fixed memory address, then loads the address of a fixed memory address into a0, then copies d0 to the memory at an offset from a0, then swaps the halves of d0, then copies d0 to the memory at an offset from a0, then returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (sub.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `frag_move_btst_bne_rts`: Short code fragment that copies a fixed memory address to a fixed memory address, then copies 0 to a fixed memory address, then copies an address to a fixed memory address, then copies 14100 to a fixed memory address, then tests bit 14 of a fixed memory address, then branches if the last result was not zero, then returns.
- `calculate_and_setup_blitter`: Calculates blitter parameters based on input values, updates memory locations with computed results, and configures multiple blitter registers for graphics operations.
- `setup_blitter_dma`: Configures multiple blitter registers with specific values and iterates through a loop to set up blitter source and destination pointers while updating counters.
- `modify_bitmasks_and_pointers`: Modifies bitmasks in memory locations, updates various data structures, and performs bitwise operations on global variables and pointer values.
- `update_pointer_table`: Updates a pointer table by calculating offsets based on input values and storing computed addresses at specific memory locations.
