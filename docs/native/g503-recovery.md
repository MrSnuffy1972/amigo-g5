# Game 503 (Gold of the Aztecs, the): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00503). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1827456 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 79 of 79 named (0 author symbols), 79 of 79 described (14 by rule, 50 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1781 of 1781 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 2560 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5828 of 5828 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 16141 |
| code | 30038 |
| data | 1756698 |
| padding | 3 |
| unused | 24576 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2).adf:bootblock | 1 | 74 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5208 | 41 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 546 | 32 |
| Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2).adf bytes 1024-3072 (code never loaded by a reference run) | 1 | 0 | 0 |
| Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2).adf bytes 25088-25600 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- gold of the aztecs, the (1990)(u. s. gold)[cr pdx](disk 1 of 2)/Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2).adf: custom-format (non-DOS) disk, 1727 non-empty blocks: matched against track-loaded segments
- gold of the aztecs, the (1990)(u. s. gold)[cr pdx](disk 2 of 2)/Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 2 of 2).adf: custom-format (non-DOS) disk, 1741 non-empty blocks: matched against track-loaded segments
- 1218 executed addresses below an address (an address) run the game's own code, not harness stubs (their opcode words are not the fake OS's): treated as game code (LOWCODE, 15 Nov 2026)
- runtime segment an address: 1548 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 233 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- gold of the aztecs, the (1990)(u. s. gold)[cr pdx](disk 1 of 2)/Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2).adf: bytes 1024-3072 look like code no run loaded: recovered position-independently (load address unknown)
- gold of the aztecs, the (1990)(u. s. gold)[cr pdx](disk 1 of 2)/Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2).adf: bytes 25088-25600 look like code no run loaded: recovered position-independently (load address unknown)
- gold of the aztecs, the (1990)(u. s. gold)[cr pdx](disk 1 of 2)/Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2).adf: 2 logged loads cover 2560 bytes of the image
- gold of the aztecs, the (1990)(u. s. gold)[cr pdx](disk 1 of 2)/Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2).adf: 47 track-loaded blocks found in recovered runtime segments and classified through them
- gold of the aztecs, the (1990)(u. s. gold)[cr pdx](disk 1 of 2)/Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2).adf: blocks 95-95 (bytes 48640-49152) outside the filesystem hold data, not code (opcode density 0.012, entropy 5.59)
- gold of the aztecs, the (1990)(u. s. gold)[cr pdx](disk 1 of 2)/Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2).adf: blocks 1749-1753 (bytes 895488-898048) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.47)
- gold of the aztecs, the (1990)(u. s. gold)[cr pdx](disk 2 of 2)/Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 2 of 2).adf: blocks 1694-1759 (bytes 867328-901120) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.99)
- 2 never-loaded region(s), 2560 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 14 packed track region(s), 1712128 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `pulse_disk_select_line_low_high` (bootblock): Saves d0, loads the disk control shadow byte, clears bit 0 and sets bit 1, writes CIAB_PRB, then writes the same value again with bit 0 set, restores d0 and branches. Kind: disk hardware timing/sync check. Pass path: bra.b an address.
- `reset_six_channels_and_enable_dma` (runtime segment an address): Copies a word within a0, stores a1 at offset 4 of a0, optionally writes audio period and volume, clears six channel flags and a long at offset 106 of a3, then enables ADKCON and DMACON bits. Kind: disk hardware/track check. Pass path: The loop continues to process data/tracks; the routine is part of a larger validation loop where failure to meet specific hardware/timing/data conditions (implied by the complex pointer arithmetic and register manipulation) would lead to a .
- `spin_short_delay_loop` (runtime segment an address): Counts a loop variable down from five hundred and twelve to zero and returns, giving a short busy-wait delay. Kind: delay loop/timing check. Pass path: the instruction immediately following the call.

## Functions


### Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2).adf:bootblock

- `bootblock_bra_to_main_entry`: Branches forward to the boot block main entry, then loads the address of a nearby data area into a0 and stores it at absolute address 32 and on the stack.
- `bootblock_init_hardware_and_copy_code`: Disables interrupts and DMA, sets the stack, clears the color register and a flag byte, calls two setup routines, copies 256 longwords to address a fixed address and jumps through offset 24 of that copy.
- `cia_floppy_motor_select_and_delay`: Tests d0 for a negative value; otherwise builds a CIAB port B motor-select bitmask from d0 and d1, writes it to the floppy control register and flag byte, then delays and returns.
- `wait_fire_released_then_step_disk`: Saves d0, reads CIAA_PRA and tests bit 4 (fire button); if clear it calls the delay and disk-step routines, then restores d0 and branches onward.
- `pulse_disk_select_line_low_high`: Saves d0, loads the disk control shadow byte, clears bit 0 and sets bit 1, writes CIAB_PRB, then writes the same value again with bit 0 set, restores d0 and branches.
- `delay_4000_loop_then_check_fire`: Saves d7, counts d7 down from 4000 in a dbra loop, restores d7 and returns; the trailing code calls itself and tests CIAA_PRA bit 5 before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_interrupt_and_dispatch_callback`: Sets bit 5 of the interrupt request register, updates a global variable with a value, and if the variable is non-zero, calls a function pointer stored at that address.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `toggle_status_byte`: Tests a bit in a register and a byte in a global variable to decide whether to set or clear a byte at a specific memory location.
- `test_and_toggle_bits`: Clears a byte at a fixed memory address, then tests various bits in register d7 to decide whether to load a word from a global variable into d7, test a byte at a global variable, and potentially store a value and toggle a byte at a global variable.
- `frag_clr_tst_bne_rts`: Short code fragment that clears a fixed memory address, then tests a fixed memory address, then branches if the last result was not zero, then returns.
- `update_structure_links`: Clears two bytes in global memory, copies a byte between two locations, calculates an offset into a table using a multiplier, and calls another routine before storing a 16-bit value into a global variable.
- `update_virtual_state_and_table`: Updates a linked structure by incrementing counters and writing values to a table, then performs bounds checking on two global variables to update them and set a status byte.
- `frag_move_movea_clr_rts`: Short code fragment that copies a0 to the stack, then copies a fixed memory address to a0, then clears the memory at a0, then copies the top of the stack to a0, then returns.
- `manage_linked_structure_entries`: Checks a global pointer to either initialize a structure with fixed values or update a linked list by incrementing a counter and writing a word to a table.
- `frag_movea_move_subq_bcs_bsr_dbra_rts`: Short code fragment that copies a fixed memory address to a6, then copies the memory at a6 to d7, then subtracts 1 from d7, then branches if the carry flag is set, then copies the memory at an offset from a6 to d6, then calls another routine, then copies an indexed entry from a6 to d6, then loops back while counting down d7, then returns.
- `process_linked_structure_list`: Iterates through a linked list of structures, decrementing a byte at offset 0 and shifting elements to fill gaps.
- `execute_virtual_instruction`: Loads a word from a table at an offset determined by a value in a register, then calls a routine at an address retrieved from a table using that word as an index.
- `frag_move_bpl_cmpi_beq_bne_adda_moveq_r`: Short code fragment that copies the memory at a0 (advancing a0) to d7, then branches if the last result was not negative, then compares d7 with 32768, then branches if the last result was zero, then compares d7 with 32769, then branches if the last result was not zero, then adds the memory at a0 to a0, then loads 0 into d7, then returns (2 times in a row).
- `update_palette_and_brightness`: Reads a value from a global variable to index into a table, then updates a secondary color table and hardware color registers by adding or subtracting specific values based on the index.
- `process_data_table_2`: Calls another routine to copy data, then iterates through a table to selectively copy words from a source table to a destination based on a bitmask, and finally calls another routine.
- `traverse_linked_structure`: Recursively traverses a linked structure by reading a count and a pointer from a table, then updates a destination address using values from the table and calls another routine.
- `clear_nested_table_data`: Iterates through a table starting at a fixed address, and for each entry, uses a value from that entry to clear a 32-bit word at an offset within a secondary table structure.
- `frag_move_movea_clr_rts_2`: Short code fragment that copies a0 to the stack, then copies a fixed memory address to a0, then clears the memory at a0, then copies the top of the stack to a0, then returns.
- `update_linked_structure`: Checks a value at a global pointer; if it is not a specific constant, it either initializes a structure with fixed values or updates a linked list by calculating new offsets and storing a word, a byte, and a longword into a new structure.
- `frag_movem_movea_adda_bra`: Short code fragment that saves registers on the stack, then copies a fixed memory address to a1, then adds d2 to a1, then jumps to another point in the code.
- `frag_move_movea_adda_st_rts`: Short code fragment that copies a1 to the stack, then copies a fixed memory address to a1, then adds d2 to a1, then copies d0 to the memory at a1, then copies d1 to the memory at an offset from a1, then copies a0 to the memory at an offset from a1, then sets a fixed memory address to all ones, then copies the top of the stack to a1, then returns.
- `frag_move_movea_adda_st_rts_2`: Short code fragment that copies a1 to the stack, then copies a fixed memory address to a1, then adds d2 to a1, then copies a0 to the memory at an offset from a1, then sets a fixed memory address to all ones, then copies the top of the stack to a1, then returns.
- `frag_movem_movea_adda_add_st_rts`: Short code fragment that saves registers on the stack, then copies a fixed memory address to a1, then adds d2 to a1, then copies several registers to or from memory, then adds d0 to d2, then adds d1 to d3, then copies several registers to or from memory, then sets a fixed memory address to all ones, then restores saved registers from the stack, then returns.
- `update_indexed_table_entries`: Increments a counter in a global variable, uses the previous counter value to write a word to a table, and then performs conditional updates to several entries in a different table based on comparisons with values at specific offsets.
- `perform_blt_and_process_table`: Initializes and executes a Blitter transfer to clear or fill a memory area, then iterates through a nested table structure to conditionally clear the high bit of a byte at a specific offset and calls another routine.
- `process_coordinate_table_and_dispatch`: Validates two coordinates against a global limit, updates global state with the results, and performs a multi-stage indirect jump through a table based on calculated offsets and a byte from the input structure.
- `setup_blitter_and_cop`: Sets up hardware registers for the bitplane and display controller, then enters a loop that writes four 16-bit values to a table starting at a location derived from a pointer in a global variable.
- `swap_and_set_cop_pointer`: Swaps two 16-bit values at specific memory locations and then, if a comparison between a value and itself fails, writes a 32-bit value from a register to the coprocessor pointer register.
- `configure_blt_transfer`: Sets up the bitplane transfer registers with specific dimensions and colors, then waits for a DMA status bit to clear before setting the DMA control register.
- `frag_movem_asl_lea_move_rts`: Short code fragment that saves registers on the stack, then shifts d0 left by 1, then loads the address of a fixed memory address into a0, then copies d1 to an indexed entry from a0, then restores saved registers from the stack, then returns.
- `copy_memory_blocks_2`: Copies bytes from a source pointed to by a0 to a destination pointed to by a1, using a value from a table to determine the number of bytes and adjusting the destination pointer through nested loops.
- `copy_memory_blocks`: Negates a byte in d3 and uses it as a loop counter to copy bytes from a source pointed to by a0 to a destination pointed to by a1, adjusting the destination pointer by 7992 and 39 after each inner loop iteration.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `copy_data_block`: Copies a block of data from a source table to a destination buffer starting at an offset of 32000 from a global pointer, repeating the operation for a fixed number of iterations.
- `update_color_tables`: Iterates through a loop to copy 16-bit color values from a source table to both the hardware color registers and a secondary color table.
- `adjust_color_brightness`: Iterates through a color table to adjust the brightness of colors by incrementing or decrementing their components based on a comparison with another table, then updates the hardware color registers.
- `frag_move_eori_rts`: Short code fragment that copies d0 to the stack, then copies a fixed memory address to d0, then exclusive-ors 65535 into d0, then copies d0 to a fixed memory address (2 times in a row), then copies the top of the stack to d0, then returns.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `read_joystick_and_button`: Reads the joystick position and a button state from hardware registers, uses the position to index into a table, and returns a combined value.
- `handle_serial_interrupt`: Processes a serial interrupt by reading a byte from the serial data register, searching through tables to identify the character, and updating various global status bits and variables before returning via an interrupt return.
- `update_counters_and_call_handler`: Increments two global 16-bit words, calls another routine, compares the result with a byte at a global variable, updates two global bytes if they differ, and then calls a routine via a pointer from a global variable if a specific byte is non-zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_registers_and_return_from_stack`: Pops d0-d3, d7 and a0-a6 from the stack with movem, then returns to the caller; it is a register restore and return sequence.
- `save_regs_call_audio_then_restore`: Pushes d0-d3, d7 and a0-a6 onto the stack, calls the audio update routine, pops those registers back and returns, with a trailing branch to other code.
- `call_silence_audio`: Saves a3 on the stack, calls another routine, and restores a3 before returning.
- `call_configure_audio`: Saves d1, d7, and a3 on the stack, calls another routine, and restores them before returning.
- `call_audio_hardware_update`: Saves the current state of several data and address registers, calls update_audio_hardware_state, restores the registers, and returns.
- `call_set_state_flags`: Saves a3 on the stack, calls another routine, and restores a3 before returning.
- `call_write_status`: Calls another routine and returns.
- `silence_audio_channels_and_clear`: Clears the state of four audio channels and writes silent volume and control values to the sound hardware.
- `set_state_flags`: Sets four state flag bytes in globals to one.
- `configure_audio_hardware`: If a global byte is non-zero, it sets a hardware register to 255, performs a loop to set bits in a register, writes that register to another hardware register, and sets a global byte.
- `write_status_bytes`: Writes the value 3 to offsets 113 and 114 of a structure pointed to by a3 and returns.
- `update_four_audio_channels_and_dma`: Loops over four audio channels, reading sample pointers and periods, writing AUDxLC, AUDxLEN, AUDxPER and AUDxVOL, and enabling channel DMA bits in DMACON.
- `advance_audio_sequence_pointer`: Reads the sequence offset and pointer from the channel structure, advances by two bytes, checks the next word and reloads the pointer and offset before branching back.
- `init_channel_from_stream_bytes`: Clears the word at offset 32 of a0, copies two bytes from the a1 stream into offsets 20 and 21 of a0, sets bit 1 at offset 0 of a0, then branches away.
- `reset_six_channels_and_enable_dma`: Copies a word within a0, stores a1 at offset 4 of a0, optionally writes audio period and volume, clears six channel flags and a long at offset 106 of a3, then enables ADKCON and DMACON bits.
- `store_stream_byte_at_a3_offset`: Copies the next byte from the a1 stream into offset 119 of a3 and then branches to another routine.
- `start_audio_channel_playback`: Copies sample pointers and lengths into the four audio channel structures, programs AUDxPER and AUDxVOL from the sample data, and starts playback for each channel.
- `branch_after_test`: Tests a byte from a table and then branches to another routine.
- `move_and_branch`: Moves a byte from a pointer to an offset of a structure and then branches to another routine.
- `update_multiple_bytes`: Moves two bytes from a pointer to specific offsets in a structure, writes zero to a third offset, and then branches to another routine.
- `extend_and_store_byte`: Moves a byte from a pointer to a register, extends it to a word, stores that word in a global variable, and then branches to another routine.
- `store_byte_from_pointer`: Moves a byte from a pointer to a global variable, moves a constant byte to another global variable, and then branches to another routine.
- `extend_and_store_byte_2`: Extends a byte from a register to a word and stores it at offset 100 of a structure in a6, then branches to another routine.
- `process_data_table`: If a byte at offset 119 of a3 is zero, it loops 16 times to read a longword and a word from a table, calculates a value by dividing an address by the word, and stores the results in a global structure before updating two pointers in a3 and setting the byte at offset 119 to 255.
- `process_scaled_table`: If a byte at offset 118 of a3 is zero, it calculates a value by multiplying a word from a table by 50 and dividing by another word, then loops once to read a longword and a word from a table, calculates a division result using an address, and stores these values into a global structure and a byte at offset 14 of a5 before setting the byte at offset 118 to 255.
- `update_audio_hardware_state`: Resets the audio hardware control register and updates a global variable with a value from a register.
- `copy_and_play_audio`: Sets audio hardware registers, enables a DMA channel, copies 12 words from one memory location to another in a loop, and then updates a structure with values from a table and calls another routine.
- `update_table_pointer`: Calls another routine, loads a byte from a constant offset, and then either branches to another routine or updates a pointer in a0 and branches to another routine.
- `play_audio_sample`: Sets audio hardware registers, enables a DMA channel, and if a byte from a structure is non-negative, updates a structure with values from a table and calls another routine; otherwise it sets a register to a constant and returns.
- `calculate_offset_values`: Adds a longword from a0 to d0, calculates a word by subtracting the half of d0 from a word at offset 8 of a0, stores the word at offset 164 of a6, and stores the new longword at offset 0 of a6.
- `spin_short_delay_loop`: Counts a loop variable down from five hundred and twelve to zero and returns, giving a short busy-wait delay.
- `enable_dma_channels`: Writes a value with bit 15 set to the DMACON register to enable the selected DMA channels, then returns to its caller.
