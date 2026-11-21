# Game 4518 (nicky boum (1992)(microids)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04518). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (907232 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 10 raw-word lines |
| Functions named and described | pass, 79 of 79 named (0 author symbols), 79 of 79 described (35 by rule, 43 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 532 of 533 (99.81%) |
| Recovered without load address (counted in proof 1, flagged) | 4096 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1757 of 1758 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1128 |
| code | 7360 |
| data | 778424 |
| unused | 120320 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Nicky Boum (1992)(Microids).adf:bootblock | 1 | 18 | 1 |
| logged disk load from Nicky Boum (1992)(Microids).adf boot-block load | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 41 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1696 | 71 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| Nicky Boum (1992)(Microids).adf bytes 822272-826368 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- nicky boum (1992)(microids)/Nicky Boum (1992)(Microids).adf: custom-format (non-DOS) disk, 1523 non-empty blocks: matched against track-loaded segments
- nicky boum (1992)(microids)/Nicky Boum (1992)(Microids).adf: the boot block reads 1656 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 27 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 504 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- nicky boum (1992)(microids)/Nicky Boum (1992)(Microids).adf: bytes 822272-826368 look like code no run loaded: recovered position-independently (load address unknown)
- nicky boum (1992)(microids)/Nicky Boum (1992)(Microids).adf: 2 logged loads cover 5752 bytes of the image
- nicky boum (1992)(microids)/Nicky Boum (1992)(Microids).adf: 3 track-loaded blocks found in recovered runtime segments and classified through them
- nicky boum (1992)(microids)/Nicky Boum (1992)(Microids).adf: blocks 1628-1758 (bytes 833536-900608) outside the filesystem hold data, not code (opcode density 0.009, entropy 7.01)
- 1 never-loaded region(s), 4096 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 4 packed track region(s), 705024 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_dma_transfer` (runtime segment an address): Sets up DMA transfer parameters including disk length, disk path, interrupt enable, interrupt request, disk sync, and audio data control registers. Kind: disk track sync check. Pass path: the branch where the sync pattern and data checksum match, leading to data loading and incrementing the sector counter.
- `initialize_disk_dma` (runtime segment an address): Initializes disk DMA parameters and sets up interrupt control based on joystick input, then updates counter and coordinate values in a data structure. Kind: disk track check. Pass path: The routine proceeds through the loop without breaking, and the check passes when `u3__sub_119C()` returns 0, allowing the loop to continue until the track loading is complete..

## Functions


### Nicky Boum (1992)(Microids).adf:bootblock

- `initialize_io_request`: Sets up an I/O request structure with command code 2 and specific data values, then performs a DoIO operation.

### logged disk load from Nicky Boum (1992)(Microids).adf boot-block load

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_tst_beq_bsr_movem_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then calls another routine, then restores saved registers from the stack, then returns.
- `handle_keyboard_interrupt`: Checks for and processes keyboard interrupt conditions by reading hardware registers, modifies a global byte value based on keyboard input, and performs vertical blanking synchronization loops.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `frag_lea_move_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies 32767 to the memory at an offset from a0, then copies a fixed memory address to a fixed memory address (2 times in a row), then copies a fixed memory address to the memory at an offset from a0, then returns.
- `frag_movea_moveq_move_adda_lea`: Short code fragment that copies a location in the code to a0, then copies a location in the code to a1, then loads 0 into d0, then copies a location in the code to d0, then adds d0 to a1, then loads the address of a location in the code into a2.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_dma_parameters`: Processes a length value through division and multiplication operations, stores the result at an offset in a table, and calls another routine to set up DMA transfer parameters.
- `update_global_flag`: Loads a byte from a table, stores an ff value at that location, tests the loaded byte for positive sign, and conditionally updates another global variable with either 1 or 0 based on its previous state.
- `initialize_dma_parameters`: Reads a byte from a global table, uses it as a shift value to calculate a word length, copies bytes from the table to a structure, and updates a pointer in the structure.
- `blit_tile_pattern`: Reads tile indices from a buffer, uses them as offsets into a table, and copies values from the table to graphics registers to configure bit-block transfers for rendering a pattern of tiles.
- `process_tile_data`: Reads tile indices from a buffer, uses them as offsets into a table, copies values from the table to another location, and updates pointers and counters in a loop until all tiles are processed.
- `process_coordinates`: Processes coordinate values by masking them with 65520, shifting right by 3 bits, and storing the results in global variables at specific offsets.
- `frag_move_rts`: Short code fragment that copies d0 to a fixed memory address, then copies d1 to a fixed memory address, then returns.
- `setup_blitter`: Configures blitter registers for graphics operations by setting up various control and data pointers, then waits for DMA to complete before returning.
- `frag_movea_bsr_rts`: Short code fragment that copies the memory at a0 to a0, then calls another routine, then returns.
- `validate_and_process_data`: Validates a specific signature value and processes data pointers based on the validation result, returning a zero or negative one value depending on the outcome.
- `frag_moveq_lsl_move_dbra_rts`: Short code fragment that loads 3 into d1, then shifts d0 left by 8, then copies the memory at a0 (advancing a0) to d0, then loops back while counting down d1, then returns.
- `process_rotated_data`: Processes rotated data values from a pointer, performs bit manipulation and conditional operations on the data, and stores bytes from the input buffer to an output buffer while adjusting pointers.
- `rotate_bytes`: Rotates bytes from a stack pointer location through a loop counter and returns.
- `rotate_and_process_data`: Performs bit rotation operations on data from a pointer, processes byte values through conditional branches, and executes a series of data manipulation operations including memory copying and arithmetic calculations.
- `copy_structure_data`: Copies data from a source structure to a destination buffer while monitoring a vertical position register, continuing until a specific marker is encountered.
- `frag_movem_move_beq_bsr_addq_bra`: Short code fragment that saves registers on the stack, then copies d0 to d4, then copies d1 to d5, then copies the memory at a1 (advancing a1) to d3, then branches if the last result was zero, then copies d4 to d0, then copies d5 to d1, then calls another routine, then adds 8 to d4, then jumps to another point in the code.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `frag_lsr_move_subq_bne_rts`: Short code fragment that shifts d0 right by 1, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then subtracts 1 from d0, then branches if the last result was not zero, then returns.
- `fill_pattern_buffer`: Fills a pattern buffer with either zero bytes or data from a lookup table based on bit flags, iterating through multiple rows and updating pointers accordingly.
- `frag_clr_adda_dbra_lsr_movem_rts`: Short code fragment that clears the memory at a2, then adds d3 to a2, then loops back while counting down d0, then shifts d2 right by 1, then adds 8000 to a3, then loops back while counting down d1, then restores saved registers from the stack, then returns.
- `configure_blitter_dma`: Sets up blitter DMA parameters for graphics rendering by configuring control registers, memory addresses, and size values, then executes multiple blitter operations.
- `update_random_seed`: Updates a random seed value by combining data from hardware registers with a constant, performing bitwise operations and division before storing the result.
- `frag_move_movea_moveq_lsl_subq_addq_dbr`: Short code fragment that copies the memory at an offset from a1 to d0, then copies the memory at an offset from a1 to a1, then loads 1 into d1, then shifts d1 left by d0, then subtracts 1 from d1, then copies the memory at a0 (advancing a0) to the memory at an offset from a1, then adds 4 to a1, then loops back while counting down d1, then returns.
- `loop_collision_processing`: Processes collision data by iterating through nested loops with decrementing counters, calling routines to handle specific operations at each level.
- `handle_collision_check`: Handles collision checking by shifting a byte value, decrementing a counter, and calling routines to process the data based on the shifted value.
- `decrement_collision_values`: Decrements collision-related values in a structure by specific amounts based on bitwise comparisons, iterating through multiple checks using shift operations.
- `increment_collision_loop`: Increments collision-related values in a structure by specific amounts using nested loops and calls other routines for processing.
- `handle_increment_check`: Processes increment checks by shifting a byte value, decrementing a counter, and calling routines to handle the data based on the shifted value.
- `increment_collision_values`: Increments collision-related values in a structure by specific amounts based on bitwise comparisons, iterating through multiple checks using shift operations.
- `configure_blitter_dma_2`: Sets up blitter DMA parameters for graphics rendering with different control values than routine R5, configuring registers and executing multiple blitter operations.
- `update_position_data_3`: Calculates and stores position-related data including coordinates, movement values, and status flags into a structure referenced by a6.
- `frag_subq_addq_bra`: Short code fragment that subtracts 2 from d6, then subtracts 2 from a0, then subtracts 2 from a1, then adds 1 to d1, then jumps to another point in the code.
- `frag_move_tst_beq_addq_bra`: Short code fragment that copies d7 to d1, then tests d4, then branches if the last result was zero, then adds 1 to d1, then jumps to another point in the code.
- `update_position_data_2`: Calculates and stores position-related data including coordinates, movement values, and status flags into a structure referenced by a6, then updates multiple fields with computed values based on global variables and input parameters.
- `frag_clr_move_moveq_sub_lsr_bra`: Short code fragment that clears the memory at an offset from a6, then copies d4 to d0, then loads 16 into d1, then subtracts d0 from d1, then loads 255 into d0, then shifts d0 right by d1, then copies d0 to the memory at an offset from a6, then jumps to another point in the code.
- `update_position_data`: Updates multiple fields in a global structure with calculated position and movement data, including coordinates, pointers, and a word value derived from input registers.
- `wait_and_update`: Waits for a bit in a global structure to become clear, then updates multiple fields in that structure with values from registers and performs arithmetic operations on pointers before looping back to wait again.
- `update_sprite_positions`: Updates position data for multiple sprites by calculating new coordinates based on input values, modifying flags in a status byte, and storing results in global memory structures.
- `frag_clr_cmp_bhi_rts`: Short code fragment that clears a fixed memory address, then compares d0 with a location in the code, then branches if the comparison was higher (unsigned), then returns.
- `read_joystick_input`: Reads joystick movement and fire button states from hardware registers, processes the data to determine directional input and button presses, and stores the result in a global variable while updating a counter based on the fire button status.
- `check_joystick_input`: Tests bit 7 of a hardware register and conditionally updates several global variables based on memory contents.
- `initialize_dma_transfer`: Sets up DMA transfer parameters including disk length, disk path, interrupt enable, interrupt request, disk sync, and audio data control registers.
- `initialize_disk_dma`: Initializes disk DMA parameters and sets up interrupt control based on joystick input, then updates counter and coordinate values in a data structure.
- `frag_addq_subi_bgt_bsr_movem_rts`: Short code fragment that adds 1 to the memory at an offset from a1, then subtracts 11 from the memory at an offset from a1, then branches if the comparison was greater, then calls another routine, then restores saved registers from the stack, then returns.
- `initialize_graphics_data`: Populates a memory region with alternating bit patterns, then processes multiple data blocks through several subroutines with varying parameters before updating pointers and counters.
- `process_graphics_data`: Performs bit manipulation operations on data from a pointer, then calls two other routines with modified values before returning.
- `frag_move_lsr_bsr_rts`: Short code fragment that copies d0 to d3, then shifts d0 right by 1, then calls another routine, then copies d3 to d0, then calls another routine (2 times in a row), then returns.
- `bitwise_mix_store_word`: Combines a 16-bit input word with a mask derived from its own low bits, optionally clears bit 15 depending on a flag byte located before the output pointer, and stores the resulting word through that pointer.
- `frag_move_btst_bne_bset_bra`: Short code fragment that copies the memory at a0 to d0, then tests bit 0 of the memory at an offset from a0, then branches if the last result was not zero, then tests bit 6 of d0, then branches if the last result was not zero, then sets bit 7 of d0, then jumps to another point in the code.
- `frag_bclr_move_rts`: Short code fragment that clears bit 7 of d0, then copies d0 to the memory at a0, then returns.
- `frag_move_moveq_eor_dbra_andi_rts`: Short code fragment that copies d2 to the stack, then loads 0 into d0, then copies the memory at a0 (advancing a0) to d2, then exclusive-ors d2 into d0, then loops back while counting down d1, then masks d0 with an address, then copies the top of the stack to d2, then returns.
- `frag_bsr_btst_bne_rts`: Short code fragment that calls another routine, then tests bit 5 of a fixed memory address, then branches if the last result was not zero, then returns.
- `frag_addq_moveq_move_dbra_rts`: Short code fragment that adds 1 to the memory at an offset from a1, then loads 255 into d0, then copies d0 to the memory at an offset from a6, then loops back while counting down d0, then returns.
- `frag_btst_bne_bset_rts_bclr`: Short code fragment that tests bit 0 of the memory at an offset from a1, then branches if the last result was not zero, then sets bit 2 of the memory at a5, then returns, then clears bit 2 of the memory at a5, then returns.
- `compare_and_update_values`: Compares two word values and conditionally calls different routines based on the comparison result, updating control flow variables.
- `scale_and_divide_coordinates`: Shifts and divides long values from memory locations, storing results as words and a byte in specific offsets of a structure.
- `frag_bset_bsr_bclr_rts`: Short code fragment that sets bit 3 of the memory at a5, then sets bit 7 of the memory at a5, then calls another routine, then clears bit 3 of the memory at a5, then calls another routine, then returns.
- `frag_bset_bclr_bsr_rts`: Short code fragment that sets bit 3 of the memory at a5, then clears bit 7 of the memory at a5, then calls another routine, then clears bit 3 of the memory at a5, then calls another routine (2 times in a row), then returns.
- `frag_moveq_bra`: Short code fragment that loads 2 into d1, then jumps to another point in the code.
- `frag_move_bra`: Short code fragment that copies 1290 to d1, then jumps to another point in the code.
- `frag_moveq_move_cmp_beq_dbra_rts`: Short code fragment that loads 86 into d1, then copies the memory at an offset from a6 to d0, then compares d0 with the memory at an offset from a6, then branches if the last result was zero, then loops back while counting down d1, then returns.
- `frag_bclr_bset_bsr_addq_rts`: Short code fragment that clears bit 1 of the memory at a5, then clears bit 0 of the memory at a5, then sets bit 0 of the memory at a5, then calls another routine, then adds 2 to the memory at an offset from a1, then returns.
- `frag_bset_bclr_bsr_subq_rts`: Short code fragment that sets bit 1 of the memory at a5, then clears bit 0 of the memory at a5, then sets bit 0 of the memory at a5, then calls another routine, then subtracts 2 from the memory at an offset from a1, then returns.
- `frag_btst_beq_bsr_bra_clr_rts`: Short code fragment that tests bit 4 of a fixed memory address, then branches if the last result was zero, then calls another routine, then jumps to another point in the code, then clears the memory at an offset from a1, then returns.
- `decrement_and_compare`: Counts down from a fixed value while testing bits and comparing bytes, returning negative one when the counter reaches zero.
- `setup_audio_dma`: Performs bit rotation operations on a register, writes to audio volume registers and DMA control registers, clears global variables, and updates several memory locations with values derived from a pointer before branching to another routine.
- `initialize_audio_and_dma`: Clears memory locations in a loop, then writes to audio volume registers, DMA control registers, and several global variables before returning.
- `frag_clr_move_rts`: Short code fragment that clears a fixed memory address (5 times in a row), then copies 15 to a fixed memory address, then returns.
- `update_audio_channels`: Updates audio channel registers based on bit tests and data from a table, then returns.
- `update_position`: Updates position coordinates based on input values and clamps them within specified ranges, then stores the results at specific offsets in a structure.
- `handle_dma_and_input`: Clears multiple DMA channels, tests bit 1 of a hardware register, and sets bit 15 of another register before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
