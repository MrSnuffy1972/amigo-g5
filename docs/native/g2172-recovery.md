# Game 2172 (great giana sisters, the (1988)(time warp)[cr hqc][t skyline]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02172). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904848 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 48 of 48 named (0 author symbols), 48 of 48 described (1 by rule, 19 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 571 of 571 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 1536 bytes, 4 functions |
| Independent decoder cross-check (Ghidra against capstone) | 975 of 975 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 682 |
| code | 3838 |
| data | 720555 |
| padding | 61 |
| unused | 179712 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf:bootblock | 1 | 143 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 73 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 586 | 31 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf bytes 1024-2560 (code never loaded by a reference run) | 1 | 166 | 4 |

## Cases handled

- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: custom-format (non-DOS) disk, 1407 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 68 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 477 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: bytes 1024-2560 look like code no run loaded: recovered position-independently (load address unknown)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: 1 logged loads cover 1536 bytes of the image
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 11-33 (bytes 5632-17408) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.07)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 44-66 (bytes 22528-34304) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.69)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 77-99 (bytes 39424-51200) outside the filesystem hold data, not code (opcode density 0.014, entropy 3.40)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 110-132 (bytes 56320-68096) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.30)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 143-165 (bytes 73216-84992) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.28)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 176-198 (bytes 90112-101888) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.43)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 209-231 (bytes 107008-118784) outside the filesystem hold data, not code (opcode density 0.015, entropy 3.43)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 242-264 (bytes 123904-135680) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.57)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 275-297 (bytes 140800-152576) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.41)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 308-330 (bytes 157696-169472) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.36)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 341-363 (bytes 174592-186368) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.51)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 374-396 (bytes 191488-203264) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.50)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 407-429 (bytes 208384-220160) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.51)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 440-462 (bytes 225280-237056) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.62)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 473-495 (bytes 242176-253952) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.35)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 506-528 (bytes 259072-270848) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.36)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 539-561 (bytes 275968-287744) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.49)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 572-594 (bytes 292864-304640) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.35)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 605-627 (bytes 309760-321536) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.59)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 638-660 (bytes 326656-338432) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.27)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 671-693 (bytes 343552-355328) outside the filesystem hold data, not code (opcode density 0.009, entropy 3.71)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 704-726 (bytes 360448-372224) outside the filesystem hold data, not code (opcode density 0.016, entropy 3.84)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 737-759 (bytes 377344-389120) outside the filesystem hold data, not code (opcode density 0.008, entropy 3.40)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 770-792 (bytes 394240-406016) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.58)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 803-825 (bytes 411136-422912) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.35)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 836-858 (bytes 428032-439808) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.50)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 869-891 (bytes 444928-456704) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.35)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 902-924 (bytes 461824-473600) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.48)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 935-957 (bytes 478720-490496) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.61)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 968-990 (bytes 495616-507392) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.39)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 1001-1023 (bytes 512512-524288) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.17)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 1034-1055 (bytes 529408-540672) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.40)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 1067-1111 (bytes 546304-569344) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.98)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 1122-1144 (bytes 574464-586240) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.92)
- great giana sisters, the (1988)(time warp)[cr hqc][t skyline]/Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf: blocks 1155-1755 (bytes 591360-899072) hold 2 packed streams (BK: ByteKiller) found by scanning, 94% of the bytes

## Cracktros (kept, not protection)

- none

## Copy protection found

- `boot_drive_seek` (bootblock): Boot loader drive set-up: selects drive zero, motor on, steps to the requested cylinder and side. Kind: track/sector verification loop. Pass path: The loop completes when sVar3 reaches -1, allowing execution to proceed to the next instruction after the function call..
- `start_track_dma_read` (bootblock): Sets the disk control and sync registers, calls an unnamed routine, points the disk DMA at a fixed buffer and sets the disk length register. Kind: track/disk validation check. Pass path: the code continues to the instruction 'lea.l an address.l, a2' and performs a comparison against the value an address.
- `boot_step_to_next_track` (bootblock): Boot loader: switches side or steps one cylinder in. Kind: track/sector validation check. Pass path: the branch where (in_D0 & 1) != 0 (the 'else' path or falling through the if-block).
- `boot_drive_off` (bootblock): Boot loader: stops the motor and deselects the drive. Kind: disk hardware/drive configuration check. Pass path: the execution continues to the next instruction (DAT_00bfd100 modification).

## Functions


### Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf:bootblock

- `write_interrupt_register_then_call`: Boot block entry, called by the Kickstart strap, that writes the interrupt enable register and then makes a single call to an unnamed routine.
- `decode_mfm_tracks_into_memory`: Selects the drive and steps through the disk, scanning a track buffer for sync-marked blocks.
- `boot_drive_seek`: Boot loader drive set-up: selects drive zero, motor on, steps to the requested cylinder and side.
- `wait_for_disk_flag`: Busy-waits while a flag bit in the disk status word stays set, then returns.
- `start_track_dma_read`: Sets the disk control and sync registers, calls an unnamed routine, points the disk DMA at a fixed buffer and sets the disk length register.
- `boot_step_to_next_track`: Boot loader: switches side or steps one cylinder in.
- `boot_drive_off`: Boot loader: stops the motor and deselects the drive.
- `stub_empty_return`: Contains only a return.
- `delay_countdown_loop`: Spins a countdown loop of about ten thousand iterations as a fixed delay, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_color_palette`: Updates a hardware color register by reading bitstream data through a bit buffer.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `execute_subroutines`: Executes a series of other routines, including a jump to a fixed address and a table-based update to a destination address, while touching a color register.
- `boot_allocate_buffers`: Hands out the boot buffers downward from the top of chip memory and derives the logo, title and music addresses.
- `update_bitplane_pointers`: Updates the memory pointers used for bitplane access by reading a bitmap header.
- `execute_subroutine_sequence`: Executes a sequence of three or more subroutines in succession.
- `execute_sequential_routines`: Executes a sequence of other routines, including a loop that continues as long as a certain global variable remains zero.
- `build_logo_copper_list`: Builds the logo copper list and sets up the display for the picture size and depth.
- `shift_and_copy_colors`: Inverts a bit in a global variable and, if not zero, shifts a block of memory forward before copying that memory to a hardware register or a table.
- `set_dma_control`: Writes a constant value to a global variable that controls DMA or a similar hardware setting.
- `execute_multiple_routines`: Executes a sequence of other routines, including a loop that continues while a certain global variable is zero.
- `build_title_copper_lists`: Builds two alternating copper lists for the title picture and starts the display.
- `boot_wait_raster_line`: Waits until the video beam reaches the line given in d6.
- `decrement_register_loop`: Decrements a register in a loop until it reaches zero.
- `boot_add_copper_pointer_pair`: Appends two copper moves that load a chip address into a pointer register pair.
- `initialize_bitplane_pointers`: Reads a bitmap header and configures the memory pointers for each bitplane in a picture buffer.
- `read_ilbm_header`: Reads the bitmap header (size, depth, compression) and finds the colour map and body chunks.
- `set_picture_plane_pointers`: Computes the start of each bitplane of the picture buffer.
- `iterate_memory_block`: Iterates through a memory range, performing a conditional copy or move and checking a byte value.
- `copy_memory_with_conditional_logic`: Performs a conditional memory copy or block move using a counter and checks a byte value to determine if a secondary copy or a different copy length is required.
- `load_picture_palette`: Converts the picture colour map to Amiga colour words and writes them to the colour registers and the cycling table.
- `boot_read_joystick`: Reads the joystick in port two and the fire button into direction and fire flags.
- `update_table_values`: Reads a value from a table and adds it to a base value, storing the result in a destination address pointed to by a table entry.
- `update_offset_table`: iterates through a null-terminated table of offsets and adds a base value to the value stored at a destination address.
- `enable_dma_channels`: Sets a specific value to the DMA controller register to enable certain DMA channels.
- `set_dma_register`: Writes a constant value to the DMA controller register.
- `initialize_sequencer_channels`: Sets up several hardware registers and global variables with specific addresses and counters, then calls another routine multiple times.
- `boot_reset_sequencer_channel`: Resets one sequencer channel record of the boot copy of the music player.
- `silence_audio_channel`: Calls another routine and then writes zero to a memory location and a value from a global variable to a memory location offset from a previously set address.
- `boot_silence_one_channel`: Silences one audio channel and writes its volume from the table.
- `boot_music_tick`: Advances the four channels of the boot copy of the music sequencer once.
- `boot_sequencer_channel_tick`: One tick of a channel in the boot copy of the Beathoven sequencer: note length, pattern commands, portamento and DMA restart.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_interrupt_enable`: Modifies the interrupt enable bit in a hardware register.

### Great Giana Sisters, The (1988)(Time Warp)[cr HQC][t Skyline].adf bytes 1024-2560 (code never loaded by a reference run)

- `open_graphics_and_build_display`: Opens the graphics library, clears about a thousand longwords of memory and fills a second buffer.
- `adjust_six_bit_counter_byte`: Adjusts a status byte in place.
- `fill_stepped_table_with_markers`: Fills a table in memory with values that step by a fixed increment, writing a marker value of minus two after each run, until a fixed table end address is reached.
- `fill_stepped_table_with_counter`: Writes a stepping table of values into memory, marking each run end with minus two.
