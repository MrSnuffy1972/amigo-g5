# Game 503 (Gold of the Aztecs, the): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00503). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1811312 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 49 of 49 named (0 author symbols), 49 of 49 described (2 by rule, 27 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 332 of 332 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 2048 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2021 of 2021 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 6001 |
| code | 10928 |
| data | 1769804 |
| padding | 3 |
| unused | 24576 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2).adf:bootblock | 1 | 74 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1401 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 546 | 32 |
| Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2).adf bytes 1024-3072 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- gold of the aztecs, the (1990)(u. s. gold)[cr pdx](disk 1 of 2)/Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2).adf: custom-format (non-DOS) disk, 1727 non-empty blocks: matched against track-loaded segments
- gold of the aztecs, the (1990)(u. s. gold)[cr pdx](disk 2 of 2)/Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 2 of 2).adf: custom-format (non-DOS) disk, 1741 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 99 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 233 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- gold of the aztecs, the (1990)(u. s. gold)[cr pdx](disk 1 of 2)/Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2).adf: bytes 1024-3072 look like code no run loaded: recovered position-independently (load address unknown)
- gold of the aztecs, the (1990)(u. s. gold)[cr pdx](disk 1 of 2)/Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2).adf: 1 logged loads cover 2048 bytes of the image
- gold of the aztecs, the (1990)(u. s. gold)[cr pdx](disk 1 of 2)/Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2).adf: 16 track-loaded blocks found in recovered runtime segments and classified through them
- gold of the aztecs, the (1990)(u. s. gold)[cr pdx](disk 1 of 2)/Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2).adf: blocks 95-95 (bytes 48640-49152) outside the filesystem hold data, not code (opcode density 0.012, entropy 5.59)
- gold of the aztecs, the (1990)(u. s. gold)[cr pdx](disk 1 of 2)/Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 1 of 2).adf: blocks 1749-1753 (bytes 895488-898048) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.47)
- gold of the aztecs, the (1990)(u. s. gold)[cr pdx](disk 2 of 2)/Gold of the Aztecs, The (1990)(U. S. Gold)[cr PDX](Disk 2 of 2).adf: blocks 1694-1759 (bytes 867328-901120) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.99)
- 1 never-loaded region(s), 2048 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 11 packed track region(s), 1728512 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

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

- `decrement_counter_and_dispatch_table`: Decrements the global byte counter, adds sixteen to a3, and if the counter was nonzero jumps through a table indexed by the counter and low nibble of a3; otherwise clears the counter and d6 high word.
- `setup_bitplane_data`: Configures several hardware registers for display and DMA, then enters a loop that writes four 32-bit values to a table by splitting them into two 16-bit words and incrementing the source value by 8000 each iteration.
- `swap_pointer_pairs_and_set_copper`: Swaps the word at address 774 with 770 and the long at 868 with 864, then compares two reads of 1030 and writes d0 to COP1LCH if they match.
- `blitter_copy_setup_with_dma_wait`: Saves d0, programs blitter pointers, modulos, masks, control and size for a copy, enables DMA, waits for bit 14 of DMACONR, then writes 1088 to DMACON and returns.
- `update_color_register`: Shifts the value in a register left by one and uses it as an offset to write the contents of another register into a color register.
- `copy_nested_blocks`: Copies bytes from one memory location to another in a loop, with nested loops that adjust the destination pointer based on a byte value, and calls another routine.
- `copy_data_with_loops`: Negates a byte in d3 and, if it is not negative, enters nested loops to copy bytes from one memory location to another using d0 and d1 as counters.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `read_joy1_fire_and_merge_status`: Saves d1, reads JOY1DAT twice until stable, extracts joystick bits, indexes a table with them, reads CIAA_PRA fire bit, stores a flag at 1277 and returns the merged value in d0.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `vblank_joystick_update_and_callback`: Increments two counters, calls the joystick read routine, stores changed joystick state at 1252 and 1209, then optionally calls the pointer at 896 before returning from interrupt.

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
