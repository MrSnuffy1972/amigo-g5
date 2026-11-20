# Game 3923 (HighwayHawks v1.0 2538): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03923). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4044964 top-level bytes in 9 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 9 of 9 named (0 author symbols), 9 of 9 described (4 by rule, 4 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 158 of 158 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 10829 |
| code | 522 |
| data | 2242577 |
| padding | 60 |
| unused | 1790976 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 15 | 5 |
| Disk.1:bootblock | 1 | 143 | 4 |
| logged disk load from Disk.1 boot-block load | 1 | 0 | 0 |

## Cases handled

- HighwayHawks_v1.0_2538/HighwayHawks_v1.0_2538.hdf: hard-disk image (AmigaDOS OFS, 4352 blocks): its filesystem parses and all 6 files in it are byte-identical to the extracted tree HighwayHawks_v1.0_2538_hdf, which is recovered file by file; no block outside the filesystem holds data
- HighwayHawks_v1.0_2538/HighwayHawks_v1.0_2538_hdf/HighwayHawks/Disk.1: the boot block reads 5632 bytes from disk offset 16896 to an address (found statically); recovered as a unit
- HighwayHawks_v1.0_2538/HighwayHawks_v1.0_2538_hdf/HighwayHawks/Disk.1: 1 logged loads cover 0 bytes of the image
- HighwayHawks_v1.0_2538/HighwayHawks_v1.0_2538_hdf/HighwayHawks/Disk.1: blocks 11-21 (bytes 5632-11264) hold ASCII text
- HighwayHawks_v1.0_2538/HighwayHawks_v1.0_2538_hdf/HighwayHawks/Disk.2: boot area holds bytes but the checksum is invalid, so Kickstart never runs it

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_cmpa_beq_jmp`: Short code fragment that compares a4 with a location in the code, then branches if the last result was zero, then jumps to another point in the code.
- `frag_lea_bsr_jmp`: Short code fragment that loads the address of the memory at an offset from a5 into a6, then calls another routine, then jumps to the address in a6.
- `frag_move_addi_rts`: Short code fragment that copies a location in the code to the stack, then adds 32 to the top of the stack, then returns.
- `frag_pea_move_addq_rts`: Short code fragment that pushes the address of a fixed memory address onto the stack, then copies a location in the code to the stack, then adds 4 to the top of the stack, then returns.

### Disk.1:bootblock

- `parse_hunk_headers_2`: Processes AmigaDOS hunk type codes from a data buffer and updates pointers in a table based on the parsed values, handling different hunk types including code, data, and relocation entries.
- `parse_hunk_headers`: Processes AmigaDOS hunk type codes from a data buffer and updates pointers in a table based on the parsed values.
- `copy_screen_data`: Allocates memory for screen buffers using a system call, then copies pixel data between memory locations in a pattern of 1760 words per iteration across multiple buffer pairs.
- `swap_bits`: Swaps adjacent bits in each word of a buffer, performing bit manipulation operations on the data pointed to by a0 register.
