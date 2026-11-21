# Game 3710 (foundation's waste (1988)(exocet software)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03710). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904912 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 24 of 24 named (0 author symbols), 24 of 24 described (9 by rule, 15 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 87 of 87 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 871 of 871 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 3364 |
| data | 730540 |
| unused | 171008 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Foundation's Waste (1988)(Exocet Software).adf:bootblock | 1 | 145 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 69 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 657 | 11 |

## Cases handled

- foundation's waste (1988)(exocet software)/Foundation's Waste (1988)(Exocet Software).adf: custom-format (non-DOS) disk, 1424 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 67 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 20 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- foundation's waste (1988)(exocet software)/Foundation's Waste (1988)(Exocet Software).adf: blocks 2-10 (bytes 1024-5632) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.19)
- foundation's waste (1988)(exocet software)/Foundation's Waste (1988)(Exocet Software).adf: blocks 22-103 (bytes 11264-53248) outside the filesystem hold data, not code (opcode density 0.060, entropy 5.92)
- foundation's waste (1988)(exocet software)/Foundation's Waste (1988)(Exocet Software).adf: blocks 105-106 (bytes 53760-54784) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.27)
- foundation's waste (1988)(exocet software)/Foundation's Waste (1988)(Exocet Software).adf: blocks 110-112 (bytes 56320-57856) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.70)
- foundation's waste (1988)(exocet software)/Foundation's Waste (1988)(Exocet Software).adf: blocks 146-220 (bytes 74752-113152) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.44)
- foundation's waste (1988)(exocet software)/Foundation's Waste (1988)(Exocet Software).adf: blocks 494-515 (bytes 252928-264192) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.74)
- foundation's waste (1988)(exocet software)/Foundation's Waste (1988)(Exocet Software).adf: blocks 517-627 (bytes 264704-321536) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.58)
- foundation's waste (1988)(exocet software)/Foundation's Waste (1988)(Exocet Software).adf: blocks 629-655 (bytes 322048-335872) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.08)
- foundation's waste (1988)(exocet software)/Foundation's Waste (1988)(Exocet Software).adf: blocks 657-1738 (bytes 336384-890368) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.74)
- foundation's waste (1988)(exocet software)/Foundation's Waste (1988)(Exocet Software).adf: blocks 1749-1759 (bytes 895488-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.53)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Foundation's Waste (1988)(Exocet Software).adf:bootblock

- `initialize_system`: Initializes system components by calling library functions, clearing hardware registers, setting up memory locations, and performing initialization routines before jumping to main code.
- `frag_movem_move_tst_bne_bsr_rts`: Short code fragment that saves registers on the stack, then copies a location in the code to d0, then tests d0, then branches if the last result was not zero, then calls another routine, then restores saved registers from the stack, then calls another routine, then returns.
- `frag_lea_move_bsr_tst_rts`: Short code fragment that loads the address of a location in the code into a5, then copies an address to the memory at a5, then copies a location in the code to d1, then copies 1024 to d0, then calls another routine, then copies a location in the code to d0, then tests d0, then returns.
- `compare_and_copy_loop`: Compares bytes from two memory locations and continues until a mismatch is found, then copies data from a table to global variables and performs a loop of memory operations.
- `frag_move_bra`: Short code fragment that copies 11264 to d0, then copies a location in the code to d1, then jumps to another point in the code.
- `call_do_i_o`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `frag_movea_clr_move_jsr_rts`: Short code fragment that copies a4 to a1, then clears the memory at an offset from a1 (3 times in a row), then copies 9 to the memory at an offset from a1, then copies a fixed memory address to a6, then calls another routine, then returns.
- `update_pointer_array`: Updates an array of pointers by adding values to their targets, processing elements until a zero value is encountered.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_bra`: Short code fragment that jumps to another point in the code.
- `handle_interrupts`: Reads interrupt status registers and sets up interrupt enable and request flags based on conditions, calling another routine and updating hardware registers.
- `update_global_counter`: Checks a global byte variable and updates a word counter based on its value, with conditional branching and subroutine calls.
- `frag_lea_moveq_move_dbra_addq_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of the memory at an offset from a0 into a1, then loads 15 into d6, then loads 9 into d7, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d7, then adds 2 to a0, then adds 2 to a1, then loops back while counting down d6, then returns.
- `character_render_loop`: Processes character data from a lookup table based on an index counter, copying 16 words to a destination buffer and handling special cases for space characters.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_initialize`: Clears specific bits in a register, reads from memory locations, performs data transfers between tables, and initializes various game state variables.
- `process_audio_data`: Processes audio data by reading from lookup tables, performing arithmetic operations, and updating audio hardware registers with calculated values.
- `clear_audio_registers`: Clears audio volume registers for four audio channels and resets specific memory locations in the custom hardware registers.
- `frag_bsr_clr_rts`: Short code fragment that calls another routine, then clears a fixed memory address (2 times in a row), then returns.
- `bit_shift_and_copy`: Shifts bits in a global variable, copies data from one location to another based on bit values, and updates a counter until it reaches eight iterations.
- `calculate_and_shift_values`: Performs arithmetic operations on values from a lookup table, calculates differences, and applies bit shifting and rotation operations to update global variables.
- `copy_lookup_data`: Copies byte values from a lookup table at an offset into multiple memory locations relative to a0 register, with conditional copying of a sixth byte.
- `setup_audio_parameters`: Loads audio parameters from memory locations, updates hardware registers for audio channels, and copies data to multiple memory destinations based on calculated offsets.
- `update_audio_periods`: Reads audio period values from a table, stores them in audio hardware registers, and updates corresponding memory locations with the same values.
- `configure_audio_dma`: Sets up audio period registers for four channels, configures DMA control settings based on global variables, and initializes volume registers for audio channels.
- `frag_clr_rts`: Short code fragment that clears d0, then clears d1, then clears d2, then clears d3, then clears d4, then clears d5, then clears d6, then clears d7, then returns.
