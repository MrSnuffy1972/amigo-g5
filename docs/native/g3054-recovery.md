# Game 3054 (CaveMania v1.1 2220): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03054). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (971138 top-level bytes in 12 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 17 of 17 named (0 author symbols), 17 of 17 described (6 by rule, 9 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 108032 bytes, 11 functions |
| Independent decoder cross-check (Ghidra against capstone) | 783 of 803 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 10878 |
| code | 2966 |
| data | 957294 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 43 | 6 |
| Cavemania01.BIN (raw code file, load address unknown) | 1 | 760 | 11 |

## Cases handled

- CaveMania_v1.1_2220/CaveMania_v1.1_2220.hdf: hard-disk image (AmigaDOS OFS, 1056 blocks): its filesystem parses and all 8 files in it are byte-identical to the extracted tree CaveMania_v1.1_2220_hdf, which is recovered file by file; no block outside the filesystem holds data
- CaveMania_v1.1_2220/CaveMania_v1.1_2220_hdf/CaveMania/CavemaniaDATA/Cavemania02.BIN: Exploder Style Data File (xfd)-packed, unpacked with ancient (125952 -> 5760 bytes)
- CaveMania_v1.1_2220/CaveMania_v1.1_2220_hdf/CaveMania/CavemaniaDATA/Cavemania03.BIN: Exploder Style Data File (xfd)-packed, unpacked with ancient (182272 -> 37696 bytes)
- CaveMania_v1.1_2220/CaveMania_v1.1_2220_hdf/CaveMania/CavemaniaDATA/Cavemania01.BIN: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 108032 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_movem_movea_jsr_bra`: Short code fragment that saves registers on the stack, then copies a location in the code to a2, then calls another routine, then jumps to another point in the code.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `frag_movem_movea_jsr_rts`: Short code fragment that saves registers on the stack, then copies a location in the code to a2, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_movem_lea_bset_bclr_move_moveq_bra`: Short code fragment that saves registers on the stack, then loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then sets bit 7 of the memory at an offset from a0, then clears bit 7 of the memory at a0, then copies 28416 to the memory at an offset from a1, then loads 0 into d0, then loads 7 into d1, then jumps to another point in the code.
- `toggle_bits_loop`: Performs a bit manipulation loop that sets and clears specific bits in memory while updating a counter, then stores the final counter value to a global variable.

### Cavemania01.BIN (raw code file, load address unknown)

- `initialize_system`: Performs system initialization by setting up stack pointer, copying data from a source buffer to a destination buffer using a loop with 1024 iterations, then calling several routines including a subroutine at offset 818 and a routine at address a fixed address.
- `parse_bitmap_data`: Processes bitmap header information and decompresses pixel data, storing decoded values into a destination buffer while handling both direct copying and run-length encoded sequences.
- `process_data_stream`: Processes a data stream by performing bitwise operations and conditional jumps based on bit values, with multiple subroutine calls and memory writes to a buffer.
- `frag_move_eor_roxr_rts`: Short code fragment that copies the memory just below a0 (moving a0 down) to d0, then exclusive-ors d0 into d5, then rotates d0 right through the extend flag by 1, then returns.
- `frag_subq_moveq_lsr_bne_move_eor_roxr_r`: Short code fragment that subtracts 1 from d1, then loads 0 into d2, then shifts d0 right by 1, then branches if the last result was not zero, then copies the memory just below a0 (moving a0 down) to d0, then exclusive-ors d0 into d5, then rotates d0 right through the extend flag by 1, then rotates d2 left through the extend flag by 1, then loops back while counting down d1, then returns.
- `copy_color_table`: Copies 32 color values from a source table to the custom color registers starting at COLOR00, then returns.
- `fill_memory_pattern`: Writes a predefined pattern of words to memory starting at the address in a0, with each pair consisting of a word value followed by a swapped version of a constant.
- `setup_audio_dma`: Configures audio registers and DMA control bits while managing interrupt settings, writing specific values to multiple audio channel volume and length registers, and setting up interrupt enable and request flags.
- `update_object_state`: Processes object state changes based on input data, updating position and velocity values while handling audio and DMA configuration.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `update_position`: Processes movement commands based on input data, updating position and velocity values in a structure, with bounds checking and clamping logic.
