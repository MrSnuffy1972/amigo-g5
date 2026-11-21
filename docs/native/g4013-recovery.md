# Game 4013 (Insects in Space): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04013). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (911984 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 33 of 33 named (0 author symbols), 33 of 33 described (10 by rule, 21 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 783 of 783 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1008 of 1008 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 60527 |
| code | 4032 |
| data | 836989 |
| padding | 708 |
| unused | 9728 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Insects in Space.adf:bootblock | 1 | 63 | 2 |
| logged disk load from Insects in Space.adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 155 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 790 | 28 |

## Cases handled

- Insects in Space/Insects in Space.adf: custom-format (non-DOS) disk, 1739 non-empty blocks: matched against track-loaded segments
- reference run ref ends at a 68000 address error (odd PC an address after an address): the harness halted there, nothing after it ran (68000 HALTED: address error at odd PC an address after an address, vector 3 an address is not a handler in RAM (the 68000 double-faults and halts); run ended at frame 75)
- Insects in Space/Insects in Space.adf: the boot block reads 55808 bytes from disk offset 318976 to an address (found statically); recovered as a unit
- runtime segment an address: 149 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 576 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Insects in Space/Insects in Space.adf: 1 logged loads cover 55808 bytes of the image
- Insects in Space/Insects in Space.adf: blocks 4-10 (bytes 2048-5632) outside the filesystem hold data, not code (opcode density 0.006, entropy 5.05)
- Insects in Space/Insects in Space.adf: blocks 22-622 (bytes 11264-318976) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.81)
- Insects in Space/Insects in Space.adf: blocks 732-770 (bytes 374784-394752) outside the filesystem hold data, not code (opcode density 0.002, entropy 0.24)
- Insects in Space/Insects in Space.adf: blocks 772-1541 (bytes 395264-789504) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.33)
- Insects in Space/Insects in Space.adf: blocks 1547-1759 (bytes 792064-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.40)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `frag_lea_bset_bclr_rts` (runtime segment an address): Short code fragment that loads the address of a fixed memory address into a0, then sets bit 3 of the memory at a0, then sets bit 7 of the memory at a0, then clears bit 3 of the memory at a0, then returns. Kind: disk hardware check. Pass path: CIAB_PRB is read and modified in sequence to check disk drive hardware state.

## Functions


### Insects in Space.adf:bootblock

- `setup_video_mode_and_copy_code`: Initializes video display registers including bitplane configurations and window coordinates, copies code from a routine to a fixed memory location, then jumps to that copied code.
- `setup_and_swap`: Initializes several hardware-related fields in a structure, performs a series of library calls to configure system settings, and then swaps the byte order of 256 words in a memory block.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_process_data`: Copies data from a source location to a destination, processes bits from input data through multiple conditional shifts and additions, and stores resulting byte values into a buffer.
- `process_bit_shift_and_store_2`: Reads a long word from a memory location, extracts its byte value, stores it as a word in another location, sets the condition code register to a specific value, performs a right rotate operation on the long word, and returns.
- `process_bit_shift_and_store`: Processes bit shifting operations on input data, calls another routine recursively based on shift results, and stores bytes from one memory location to another with specific offset calculations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_graphics`: Clears a memory region from a fixed address to a fixed address by writing zero values, then configures hardware registers including DMA control and interrupt settings.
- `frag_lea_move_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then copies 21 to d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then returns.
- `check_and_patch_memory`: Reads a byte from a fixed memory location and conditionally patches several memory locations with specific 32-bit and 16-bit values based on whether that byte equals one, then jumps to another address.
- `frag_lea_bset_bclr_rts`: Short code fragment that loads the address of a fixed memory address into a0, then sets bit 3 of the memory at a0, then sets bit 7 of the memory at a0, then clears bit 3 of the memory at a0, then returns.
- `setup_pointers_and_values`: Loads a base address into a register, stores word values at several memory locations relative to that address, and returns zero after performing several arithmetic operations on the base value.
- `frag_move_addi_swap_moveq_rts`: Short code fragment that copies an address to d0, then copies d0 to a fixed memory address, then adds 16896 to d0, then copies d0 to a fixed memory address, then swaps the halves of d0, then copies d0 to a fixed memory address (2 times in a row), then loads 0 into d0, then returns.
- `open_graphics_library`: Calls system functions to forbid interrupts and open the graphics library, then sets up hardware registers including DMA control and interrupt enable bits for video display.
- `handle_input_and_update_state`: Saves all general-purpose and address registers to the stack, reads a byte from a fixed memory location, compares it with specific values, branches based on the comparison results, restores registers from the stack, checks a bit in another fixed memory location, and returns immediately if the bit is not set.
- `frag_lea_bsr_move`: Short code fragment that loads the address of a fixed memory address into a0, then calls another routine, then copies an address to d0, then copies an address to d1, then copies 6 to d2, then copies 10 to d3.
- `update_and_store_values`: Copies register values to memory locations, performs arithmetic operations on those values in a loop, and stores the results with specific patterns of constants and values.
- `frag_move_sub_add_dbra_rts`: Short code fragment that copies an address to d0, then copies an address to d1, then copies 6 to d2, then copies d0 to the memory at a0 (advancing a0), then copies d1 to the memory at a0 (advancing a0), then subtracts a fixed memory address from d1, then adds a fixed memory address to d0, then copies d0 to the memory at a0 (advancing a0), then loops back while counting down d2, then returns.
- `frag_cmpi_bne_bsr`: Short code fragment that compares a fixed memory address with 255, then branches if the last result was not zero, then calls another routine.
- `frag_cmpi_bne_bsr_2`: Short code fragment that compares a fixed memory address with 254, then branches if the last result was not zero, then calls another routine, then compares a fixed memory address with 253, then branches if the last result was not zero, then calls another routine.
- `frag_cmpi_bne`: Short code fragment that compares a fixed memory address with 252, then branches if the last result was not zero.
- `frag_bsr_movem_rts`: Short code fragment that calls another routine, then saves registers on the stack, then calls another routine, then restores saved registers from the stack, then saves registers on the stack, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_tst_beq_subq_bne_bsr_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then subtracts 1 from a fixed memory address, then branches if the last result was not zero, then tests a fixed memory address, then branches if the last result was not zero, then calls another routine (2 times in a row), then returns.
- `process_input_and_update_counters`: Decrements a counter at a fixed memory location and loops until it reaches zero, then processes several conditional updates to various memory locations based on comparisons with fixed values.
- `update_counters`: Decrements multiple word-sized counters located at specific memory addresses and performs conditional arithmetic operations on them based on their current values.
- `copy_data_blocks`: Copies data from a source memory block to multiple destination locations while incrementing pointers and updating loop counters until reaching a predefined limit.
- `initialize_blitter`: Configures several Amiga custom hardware registers related to blitter operations, sets up control values for graphics rendering, and updates a state counter based on conditional comparisons.
- `copy_memory_blocks`: Copies bytes from one memory block to another at specific offsets, writing data from a source address to multiple destination addresses with fixed byte offsets.
- `generate_and_store_pattern_data`: Generates pseudo-random pattern data using hardware register values and stores it in three memory blocks, then sets up several memory locations with specific addresses and clears other memory areas.
- `increment_byte_values`: Increments byte values at specific offsets within three memory blocks and handles keyboard input by reading from the CIA serial register and updating corresponding flags.
- `init_music_sample_table`: Searches a byte table for the smallest value, then builds a pointer table by linking zeroed entries to sample start addresses indexed from a second table, sets bit 1 of a global variable, clears several global variables, and returns.
- `update_audio_channels`: Processes audio data for multiple channels by loading values from memory tables, performing bit operations on byte values, and writing results to hardware registers controlling audio output.
- `update_sprite_data`: Processes movement commands based on input data, updating position and velocity values in a structure.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `update_position`: Processes movement commands based on input data, updating position and velocity values in a structure.
