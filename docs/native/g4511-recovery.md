# Game 4511 (neverending story ii, the - the arcade game (1990)(linel)[cr qtx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04511). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (913584 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 28 raw-word lines |
| Functions named and described | pass, 121 of 121 named (0 author symbols), 121 of 121 described (44 by rule, 73 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 988 of 988 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 1536 bytes, 9 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2990 of 2990 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 3011 |
| code | 12366 |
| data | 898205 |
| padding | 2 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Neverending Story II, The - The Arcade Game (1990)(Linel)[cr QTX].adf:bootblock | 1 | 206 | 15 |
| logged disk load from Neverending Story II, The - The Arcade Game (1990)(Linel)[cr QTX].adf boot-block load | 1 | 94 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 329 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 28 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1993 | 87 |
| Neverending Story II, The - The Arcade Game (1990)(Linel)[cr QTX].adf bytes 1024-2560 (code never loaded by a reference run) | 1 | 340 | 9 |

## Cases handled

- neverending story ii, the - the arcade game (1990)(linel)[cr qtx]/Neverending Story II, The - The Arcade Game (1990)(Linel)[cr QTX].adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- neverending story ii, the - the arcade game (1990)(linel)[cr qtx]/Neverending Story II, The - The Arcade Game (1990)(Linel)[cr QTX].adf: the boot block reads 5632 bytes from disk offset 895488 to an address (found statically); recovered as a unit
- runtime segment an address: 103 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 21 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 864 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- neverending story ii, the - the arcade game (1990)(linel)[cr qtx]/Neverending Story II, The - The Arcade Game (1990)(Linel)[cr QTX].adf: bytes 1024-2560 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 immediate
- neverending story ii, the - the arcade game (1990)(linel)[cr qtx]/Neverending Story II, The - The Arcade Game (1990)(Linel)[cr QTX].adf: 2 logged loads cover 7168 bytes of the image
- 1 packed track region(s), 892928 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_cia_ports` (bootblock): Sets CIA port B data direction register to 143, writes 3 to a global variable, and configures CIA port B output to 115. Kind: disk track check. Pass path: the branch where uVar5 == *extraout_A1 and unaff_D2w == *(char *)(extraout_A1 + 1) is satisfied, leading to the `goto u0__sub_0274`.
- `frag_subq_bset_bclr_move_dbra_btst_bne` (bootblock): Short code fragment that subtracts 1 from d7, then sets bit 1 of a fixed memory address, then clears bit 0 of a fixed memory address, then sets bit 0 of a fixed memory address, then copies 3000 to d6, then loops back while counting down d6, then tests bit 5 of a fixed memory address, then branches if the last result was not zero, then jumps to another point in the code. Kind: disk hardware check. Pass path: branch where ((char)unaff_D2w == *(char *)(in_A1 + 1)) && (in_D0w == *in_A1) is true and execution reaches u0__sub_0274.
- `frag_move_tst_beq_nop_bra` (bootblock): Short code fragment that copies d2 to the memory at an offset from a1, then tests d2, then branches if the last result was zero, then copies 121 to a fixed memory address, then pauses for one no-operation step (2 times in a row), then copies 113 to a fixed memory address, then jumps to another point in the code. Kind: disk hardware check. Pass path: the routine proceeds to load data and set up disk transfer when the CIAB_PRB checks pass and the disk sync/track data matches expected values.
- `initialize_disk_operation` (bootblock): Sets up disk drive parameters and waits for a specific interrupt flag before proceeding with data transfer operations. Kind: disk track sync check. Pass path: the branch where (sVar6 != *extraout_A1_02) // (unaff_D2w != extraout_A1_02[1]) is true, leading to u0__l0_00E0, and the loop continues without entering the main data processing block.
- `initialize_audio_system` (runtime segment an address): Initializes audio system registers and memory locations by comparing data against fixed values, clearing memory blocks, and setting up interrupt configurations. Kind: disk hardware check. Pass path: The routine initializes audio system and sets up ADKCON register, indicating a disk drive hardware check (likely sync or track validation) that must pass for the program to proceed normally..
- `update_disk_status` (Neverending Story II, The - The Arcade Game (1990)(Linel)[cr QTX].adf bytes 1024-2560 (code never loaded by a reference run)): Updates disk drive status bytes based on a global variable's value, with conditional bit manipulation and delay loops, then configures hardware registers including disk control and interrupt settings. Kind: disk hardware sync/check. Pass path: The routine successfully reads the disk and updates the status, with the check passing when the correct track data is read and matches expected values (indicated by the loop breaking on successful match and setting _DAT_0000098a)..

## Functions


### Neverending Story II, The - The Arcade Game (1990)(Linel)[cr QTX].adf:bootblock

- `frag_bsr_move`: Short code fragment that calls another routine, then copies a0 to a fixed memory address, then copies 9984 to the status register.
- `copy_and_execute_code`: Copies code from one memory location to another using a loop that transfers longwords, then jumps to the copied code to execute it.
- `initialize_cop1`: Sets the COP1LCH hardware register to point to a control structure, initializes multiple longword fields in that structure, and then branches to another routine before entering an infinite loop.
- `initialize_cia_ports`: Sets CIA port B data direction register to 143, writes 3 to a global variable, and configures CIA port B output to 115.
- `frag_subq_bset_bclr_move_dbra_btst_bne`: Short code fragment that subtracts 1 from d7, then sets bit 1 of a fixed memory address, then clears bit 0 of a fixed memory address, then sets bit 0 of a fixed memory address, then copies 3000 to d6, then loops back while counting down d6, then tests bit 5 of a fixed memory address, then branches if the last result was not zero, then jumps to another point in the code.
- `frag_move_tst_beq_nop_bra`: Short code fragment that copies d2 to the memory at an offset from a1, then tests d2, then branches if the last result was zero, then copies 121 to a fixed memory address, then pauses for one no-operation step (2 times in a row), then copies 113 to a fixed memory address, then jumps to another point in the code.
- `initialize_disk_operation`: Sets up disk drive parameters and waits for a specific interrupt flag before proceeding with data transfer operations.
- `frag_swap_move_andi_lsr_cmp_beq_bra`: Short code fragment that swaps the halves of d0, then copies d0 to d1, then masks d0 with 255, then shifts d0 right by 1, then compares d0 with d7, then branches if the last result was zero, then copies d0 to d7, then copies 65535 to the memory at an offset from a1, then jumps to another point in the code.
- `update_graphics_and_status`: Updates graphics data and status flags based on table comparisons and bit manipulations, then performs conditional operations on CIA port B.
- `frag_move_bra`: Short code fragment that copies 65535 to the memory at an offset from a1, then jumps to another point in the code.
- `check_and_update_status`: Compares values from a table with registers, updates a global variable when they don't match, and performs bit manipulation on CIA port B based on conditional checks.
- `shift_and_store_words`: Processes 128 words from a source table by shifting bits and storing results in a destination table, then updates multiple pointers and returns.
- `calculate_division_result`: Performs a division operation on a value from memory, shifts the result, and returns a combined value with a conditional adjustment based on comparison.
- `frag_move_andi_lsl_or_rts`: Short code fragment that copies the memory at an offset from a0 to d0, then copies the memory at an offset from a0 to d1, then masks d0 with an address, then masks d1 with an address, then shifts d0 left by 1, then merges d1 into d0, then returns.
- `configure_dma_transfer`: Sets up a DMA transfer by configuring command and memory pointers, then executes the transfer using a library call, followed by restoring a hardware register to zero.

### logged disk load from Neverending Story II, The - The Arcade Game (1990)(Linel)[cr QTX].adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_input_states`: Processes input states through multiple conditional branches based on a global variable value, calling other routines and performing arithmetic operations on the global variable before repeating the loop.
- `wait_for_input`: Waits for specific joystick input conditions by polling a hardware register, then processes character input from a global buffer until a termination condition is met.
- `frag_jsr_lea_bra`: Short code fragment that calls another routine, then loads the address of a fixed memory address into a1, then calls another routine (2 times in a row), then jumps to another point in the code.
- `skip_special_chars`: Reads bytes from a global memory location and skips specific character values, including control codes and letters in certain ranges, before returning.
- `process_character_data`: Reads a byte from a source pointer, processes it through a lookup table, and performs arithmetic operations to compute a final result.
- `initialize_and_call_routines`: Sets up a global variable to zero, calls another routine multiple times, then sets the global variable to specific values before calling another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_subi_ori_rts`: Small routine of 3 instructions that subtracts 185 from -(a4), then merges 92 into d1, then returns.
- `setup_memory_addresses`: Loads several memory addresses into registers, performs arithmetic operations on them, stores results in memory locations, and calls several routines with parameters.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `handle_audio_interrupt`: Processes audio interrupt requests by checking hardware status, updating global flags, and managing audio channel volumes before returning.
- `handle_keyboard_input`: Checks for keyboard interrupt, reads a byte from the keyboard register, processes it through bit manipulation, stores result in a table, and updates several hardware registers.
- `initialize_audio_system`: Initializes audio system registers and memory locations by comparing data against fixed values, clearing memory blocks, and setting up interrupt configurations.
- `compare_and_copy_data`: Compares data between two memory locations, performs conditional operations based on comparison results, and copies bytes from one location to another when conditions are met.
- `frag_adda_bra`: Short code fragment that adds the memory at a2 (advancing a2) to a2, then jumps to another point in the code.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `frag_lea_move_cmpa_beq_movem_cmp_adda_b`: Short code fragment that loads the address of a location in the code into a1, then copies an address to d0, then compares a0 with a fixed memory address, then branches if the last result was zero, then copies several registers to or from memory, then compares d1 with d0, then branches if the last result was zero, then adds d2 to a0, then jumps to another point in the code.
- `frag_move_adda_bra`: Short code fragment that copies a0 to the memory at a1 (advancing a1), then adds d2 to a0, then jumps to another point in the code.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `load_and_compare_addresses`: Loads addresses into registers, compares a value with memory contents at an address, and branches based on the comparison result.
- `frag_tst_bne_lea_clr_bra`: Short code fragment that tests the memory at an offset from a2, then branches if the last result was not zero, then loads the address of the memory at an offset from a2 into a2, then clears the memory at a1 (advancing a1), then jumps to another point in the code.
- `frag_move_lea_adda_bra`: Short code fragment that copies a0 to the memory at a1 (advancing a1), then loads the address of the memory at an offset from a2 into a2, then adds d2 to a0, then jumps to another point in the code.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `configure_audio_registers`: Sets up several audio hardware registers with specific values, clears certain flags, and enables DMA channels for audio processing.
- `update_audio_counters`: Increments and manages audio system counters, performs conditional arithmetic operations on memory values, and copies data between memory locations based on comparison results.
- `update_audio_system`: Updates audio system registers by processing sound parameters from memory locations, performing arithmetic operations on counters, and copying 32-bit audio data between structures while modifying register values.
- `update_sound_registers`: Updates audio register values by processing sound parameters from multiple memory locations and performing arithmetic operations on counters.
- `process_sound_parameters`: Reads data from memory tables, performs arithmetic operations on sound parameters, updates audio registers, and stores computed values in memory locations.
- `update_audio_periods`: Loads audio period values from a table, checks bits in a register, and writes specific values to audio period registers based on bit conditions.
- `copy_audio_registers`: Copies 32-bit audio data from a source structure to a destination structure, clearing the source fields after copying.
- `update_audio_pointers`: Updates audio system pointers and executes a looped routine three times, modifying register values and memory addresses.
- `execute_sound_command`: Uses byte values from a structure to index into a command table and executes the corresponding function pointer, with special handling for negative indices.
- `frag_sub_bpl_move_rts`: Short code fragment that subtracts d1 from the memory at an offset from a3, then branches if the last result was not negative, then copies 113 to the memory at an offset from a3, then copies the memory at an offset from a3 to the memory at an offset from a4, then returns.
- `frag_add_bpl_move_rts`: Short code fragment that adds d1 to the memory at an offset from a3, then branches if the last result was not negative, then copies 113 to the memory at an offset from a3, then copies the memory at an offset from a3 to the memory at an offset from a4, then returns.
- `frag_move_bne_andi_lsr_sub_bra`: Short code fragment that copies the memory at an offset from a3 to d2, then copies a fixed memory address to d0, then copies a location in the code to d0, then branches if the last result was not zero, then masks d1 with 240, then shifts d1 right by 4, then subtracts d1 from d2, then jumps to another point in the code.
- `frag_subq_bne_move_bra`: Short code fragment that subtracts 1 from d0, then branches if the last result was not zero, then copies d2 to the memory at an offset from a4, then jumps to another point in the code.
- `frag_andi_add_bra`: Short code fragment that masks d1 with 15, then adds d1 to d2, then jumps to another point in the code.
- `check_input_flags`: Tests specific bits in registers and performs conditional jumps based on the results, with a loop that processes multiple input checks.
- `frag_subq_bne_andi_add_bra`: Short code fragment that subtracts 1 from d0, then branches if the last result was not zero, then masks d1 with 15, then adds d1 to d2, then jumps to another point in the code.
- `update_tile_position`: Adjusts a tile position value based on input parameters, clamps it between zero and 35, then writes the indexed value to two memory locations.
- `frag_move_sub_bra`: Short code fragment that copies the memory at an offset from a3 to d2, then subtracts d1 from d2, then copies d2 to the memory at an offset from a3, then jumps to another point in the code.
- `frag_move_add_bra`: Short code fragment that copies the memory at an offset from a3 to d2, then adds d1 to d2, then copies d2 to the memory at an offset from a3, then jumps to another point in the code.
- `frag_tst_beq_andi_move_rts`: Short code fragment that tests d1, then branches if the last result was zero, then masks d1 with 15, then copies d1 to a fixed memory address, then returns.
- `frag_tst_sne_rts`: Short code fragment that tests d1, then sets a fixed memory address if the last result was not zero, then returns.
- `modify_byte_value`: Updates a byte value at a calculated memory address by adding or subtracting another byte value, with clamping behavior for both operations.
- `trigger_audio_interrupt`: Checks if specific audio data is present in a structure and triggers an interrupt by writing to hardware registers, setting up audio channel parameters.
- `frag_move_andi_lsr_mulu_add_cmp_bge_rts`: Short code fragment that copies d1 to d0, then masks d0 with 15, then shifts d1 right by 4, then multiplies d1 by 10 (unsigned), then adds d1 to d0, then compares d0 with a fixed memory address, then branches if the comparison was greater or equal, then copies d0 to a fixed memory address, then returns.
- `clear_audio_interrupts`: Reads interrupt status flags from hardware registers, clears matching audio channel length registers, and enables corresponding audio interrupts.
- `frag_lea_adda_moveq_move_bsr_clr_rts`: Short code fragment that loads the address of a fixed memory address into a0, then adds a fixed memory address to a0, then loads 0 into d0, then copies the memory at a0 to d0, then calls another routine, then copies a0 to a fixed memory address, then clears a fixed memory address, then returns.
- `frag_lea_adda_moveq_move`: Short code fragment that loads the address of a fixed memory address into a0, then adds a fixed memory address to a0, then loads 0 into d0, then copies the memory at a0 to d0.
- `frag_lea_add_movea_move_rts`: Short code fragment that loads the address of a fixed memory address into a0, then adds d0 to d0 (2 times in a row), then copies an indexed entry from a0 to a0, then copies the memory at a0 (advancing a0) to d0, then returns.
- `frag_movem_move_cmp_beq_dbra_rts`: Short code fragment that saves registers on the stack, then copies 4 to d1, then copies a fixed memory address to d0, then compares d0 with a fixed memory address, then branches if the last result was zero, then loops back while counting down d1, then restores saved registers from the stack, then returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (addq.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (ori.w) and then falls through into the code that follows, with no return of its own.
- `decrement_volume_values`: Decrements volume values in a structure and writes the results to hardware volume registers for four audio channels, updating a global flag based on whether all values are zero.
- `frag_movea_lea_jsr_rts`: Short code fragment that copies a fixed memory address to a0, then copies a fixed memory address to a1, then loads the address of a fixed memory address into a6, then calls another routine, then returns.
- `frag_st_tst_bne_rts`: Short code fragment that sets a fixed memory address to all ones, then tests a fixed memory address, then branches if the last result was not zero, then returns.
- `swap_and_increment_values`: Swaps word portions of a long value, copies it to multiple memory locations, and performs arithmetic operations with constants to update additional memory locations.
- `populate_structure_fields`: Writes a word value and its swapped version to alternating memory locations in a structure, then adds a constant value to the original for subsequent writes.
- `populate_structure_fields_2`: Writes a word value and its swapped version to alternating memory locations in a structure, then adds a different constant value to the original for subsequent writes.
- `update_position_words`: Writes word values from a register to multiple offsets in a structure through a pointer, with alternating high and low bytes of the register value and incremented values.
- `populate_structure_data`: Copies data from a global variable to a structure at various offsets, using a loop counter and incrementing values based on a fixed offset.
- `frag_tst_bne_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then returns.
- `copy_pattern_bytes`: Copies bytes from a table to memory locations with specific offsets, using a loop that decrements a counter and shifts bits in a register.
- `transfer_pattern_data`: Transfers byte values from a lookup table to memory addresses with calculated offsets, iterating through a loop with decrementing counter and bit shifting.
- `frag_movem_move_lea_dbra_rts`: Short code fragment that saves registers on the stack, then copies 31 to d0, then loads the address of a fixed memory address into a1, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loads the address of the memory at an offset from a1 into a1, then loops back while counting down d0, then restores saved registers from the stack, then returns.
- `process_coordinate_deltas`: Processes coordinate differences by comparing and modifying word values from two arrays, storing results in another array with multiple conditional adjustments.
- `adjust_word_values`: Copies word values from one memory location to another, then compares and adjusts each word value using bitwise AND operations and conditional arithmetic adjustments.
- `compare_and_adjust_words`: Compares word values from memory locations with bitwise AND operations, performs conditional arithmetic adjustments, and branches based on comparison results to modify register values.
- `compare_and_adjust_values`: Performs arithmetic operations on word values from memory locations, compares results, and branches based on the comparison outcomes to adjust control flow.
- `update_memory_blocks`: Updates memory locations with computed values through arithmetic operations and loops, including memory block initialization and pointer adjustments.
- `frag_movea_adda_move_lea_dbra_rts`: Short code fragment that copies a fixed memory address to a1, then adds 4434 to a1, then copies 27 to d0, then copies the memory at a0 (advancing a0) to the memory at a1, then copies the memory at a0 (advancing a0) to the memory at an offset from a1 (2 times in a row), then loads the address of the memory at an offset from a1 into a1, then loops back while counting down d0, then returns.
- `initialize_memory_blocks`: Initializes memory blocks at specific addresses with zero values, using a loop to write to multiple locations with fixed offsets from a base address.
- `frag_tst_beq_lea_bsr_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then loads the address of a fixed memory address into a0, then calls another routine, then returns.
- `frag_move_movea_lea_jsr_rts`: Short code fragment that copies 0 to a fixed memory address (2 times in a row), then copies a fixed memory address to a0, then copies a fixed memory address to a1, then loads the address of a fixed memory address into a6, then calls another routine, then copies 31 to a fixed memory address, then returns.
- `check_input_flags_2`: Tests a global variable and performs conditional operations based on CIAA_PRA joystick/mouse button states, updating flags in global variables accordingly.
- `update_blit_parameters`: Updates blitter parameters by calculating memory addresses from table values and performing arithmetic operations to set up pattern transfers.
- `blit_pattern_sequence`: Performs multiple bit-block transfers using pattern data from a table, updating source and destination pointers while setting up blitter registers for each transfer.
- `adjust_pattern_offsets`: Adjusts pattern data offsets based on global variables and performs bit manipulation on coordinate values before calling another routine to process blitter operations.
- `process_blit_operations`: Processes blitter operations by checking flags, updating memory locations, and performing arithmetic calculations on register values before calling a blitting routine.
- `copy_word_pairs`: Copies word pairs from two source tables to two destination tables while performing bit rotation operations on the values.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `rotate_word_pair`: Processes a sequence of word pairs by rotating bits through a carry flag and storing the results in a destination table.
- `rotate_long_pair`: Processes a sequence of longword pairs by rotating bits through a carry flag and storing the results in a destination table.
- `copy_structured_data`: Copies structured data between memory locations with specific offsets and clears certain fields during the copy operation.
- `read_joystick_input`: Reads joystick input from hardware registers and combines it with button states to produce a combined input value.
- `update_and_copy_pattern`: Increments a global counter in a loop that runs ten times, calling another routine each iteration while updating memory addresses and values.
- `initialize_and_loop`: Initializes multiple memory addresses with a fixed value, then performs a loop of ten iterations where it calls another routine, decrements a global variable, adjusts a pointer, and continues until the loop counter reaches zero.
- `copy_pattern_data`: Copies graphical pattern data between memory locations using multiple offset calculations and performs repeated copy operations based on a counter.
- `copy_memory_blocks`: Copies memory blocks between source and destination addresses using multiple offset calculations and performs repeated copy operations.
- `initialize_pattern_data`: Initializes pattern data by copying byte values from two source tables to corresponding offsets in two destination areas, with looping logic for repeated initialization.
- `initialize_pattern_buffers`: Initializes multiple memory regions with specific bit patterns using a loop that processes 256 iterations, setting some locations to zero and others to all ones while updating global variables.
- `frag_move_movea_lea_jsr_rts_2`: Short code fragment that copies 0 to a fixed memory address, then copies 3 to a fixed memory address, then copies a fixed memory address to a0, then copies a fixed memory address to a1, then loads the address of a fixed memory address into a6, then calls another routine, then copies 31 to a fixed memory address, then returns.
- `frag_move_lea_bra`: Short code fragment that copies a fixed memory address to a fixed memory address, then loads the address of a fixed memory address into a3, then jumps to another point in the code.
- `render_sprite_pixels`: Processes sprite pixel data from a buffer, performs bit manipulation and memory writes to render pixels on screen, and updates position coordinates based on lookup table values.
- `blit_pattern_copy`: Performs multiple bit-block transfers using hardware registers to copy graphical patterns, with conditional logic based on a global variable controlling which transfers occur.
- `configure_blitter`: Sets up blitter registers for bit block transfer operations based on input parameters and global state, then performs multiple blitter operations with varying configurations.

### Neverending Story II, The - The Arcade Game (1990)(Linel)[cr QTX].adf bytes 1024-2560 (code never loaded by a reference run)

- `frag_ori_bra`: Short code fragment that merges 243 into d0, then merges 0 into d0, then merges 1 into d0, then merges 0 into d0 (2 times in a row), then merges 119 into d0, then merges 233 into d0, then merges 119 into d0, then jumps to another point in the code.
- `save_register_state`: Saves multiple register values to specific global memory locations, then calls another routine to update disk status information.
- `update_disk_status`: Updates disk drive status bytes based on a global variable's value, with conditional bit manipulation and delay loops, then configures hardware registers including disk control and interrupt settings.
- `decode_bit_sequence`: Decodes a sequence of bits by shifting and rotating register values, performs conditional operations based on bit states, and recursively processes data segments.
- `frag_lsr_beq_rts_move_roxr`: Short code fragment that shifts d5 right by 1, then branches if the last result was zero, then returns, then copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then returns.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs one instruction (subq.w) and then falls through into the code that follows, with no return of its own.
- `process_bit_sequence`: Processes a sequence of bits by shifting and rotating values in registers, performing conditional operations based on bit states, and recursively calling itself with modified parameters until a counter reaches zero.
- `scale_coordinate`: Reads a global variable, shifts it right by nine bits, performs unsigned division by twenty-two, extracts the quotient and remainder, and returns them as a 32-bit value where the upper word is the quotient and the lower word is the remainder.
- `configure_hardware_registers`: Configures several hardware registers including CIAA_PRA and COP1LCH, sets up memory addresses for various system components, and performs bit manipulation operations on data from a memory location before returning.
