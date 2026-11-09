# Game 81 (ChaosEngine v1.2 AGA 1324): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00081). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1808073 top-level bytes in 50 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 16 of 16 units identical (bytes and relocations), 10 raw-word lines |
| Functions named and described | pass, 149 of 149 named (0 author symbols), 149 of 149 described (9 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1834 of 1834 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 18432 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3265 of 3280 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 435610 |
| code | 11422 |
| data | 1347978 |
| padding | 7 |
| unused | 13056 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| ChaosEngineAGA.Slave | 1 | 325 | 14 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2121 | 87 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 415 | 24 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 400 | 16 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 5 |
| Disk.1 bytes 0-512 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 86528-86784 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.2 bytes 0-1280 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.2:chunks 2116-2148 (bytes 541696-550144)#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.1: raw disk image (913920 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: raw disk image (829440 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- 5 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 1161 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 119 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 239 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.1: bytes 0-512 look like code no run loaded: recovered position-independently (load address unknown)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.1: bytes 86528-86784 look like code no run loaded: recovered position-independently (load address unknown)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: bytes 0-1280 look like code no run loaded: recovered position-independently (load address unknown)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.1: 7 logged loads cover 414464 bytes of the image
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.1: chunks 2-5 (bytes 512-1536) outside the filesystem hold data, not code (opcode density 0.049, entropy 5.56)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.1: chunks 322-329 (bytes 82432-84480) hold ASCII text
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.1: chunks 339-840 (bytes 86784-215296) hold 3 packed streams (RNC1: Rob Northen RNC1 Compressor) found by scanning, 99% of the bytes
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.1: chunks 854-1288 (bytes 218624-329984) hold a RNC1: Rob Northen RNC1 Compressor stream (111360 -> 115052 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.1:chunks 854-1288 (bytes 218624-329984)#unpacked: high-entropy data file (7.87 bits/byte), classified as packed data
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.1: chunks 1290-1674 (bytes 330240-428800) hold a RNC1: Rob Northen RNC1 Compressor stream (98560 -> 44856 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.1: chunks 2956-3552 (bytes 756736-909568) hold a RNC1: Rob Northen RNC1 Compressor stream (152832 -> 165426 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.1: chunks 3554-3560 (bytes 909824-911616) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.97)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.1: chunks 3565-3569 (bytes 912640-913920) outside the filesystem hold data, not code (opcode density 0.017, entropy 6.59)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: 1 logged loads cover 1280 bytes of the image
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 8-24 (bytes 2048-6400) hold a RNC1: Rob Northen RNC1 Compressor stream (4352 -> 15580 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 26-1200 (bytes 6656-307456) hold a RNC1: Rob Northen RNC1 Compressor stream (300800 -> 80716 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 1202-1590 (bytes 307712-407296) hold a RNC1: Rob Northen RNC1 Compressor stream (99584 -> 48640 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 1592-1600 (bytes 407552-409856) outside the filesystem hold data, not code (opcode density 0.009, entropy 7.17)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 1602-1692 (bytes 410112-433408) hold a RNC1: Rob Northen RNC1 Compressor stream (23296 -> 40000 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 1694-1786 (bytes 433664-457472) hold a RNC1: Rob Northen RNC1 Compressor stream (23808 -> 25088 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 1788-1868 (bytes 457728-478464) hold a RNC1: Rob Northen RNC1 Compressor stream (20736 -> 25088 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 1870-1910 (bytes 478720-489216) hold a RNC1: Rob Northen RNC1 Compressor stream (10496 -> 25088 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 1912-1950 (bytes 489472-499456) hold a RNC1: Rob Northen RNC1 Compressor stream (9984 -> 20480 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 1952-2042 (bytes 499712-523008) hold a RNC1: Rob Northen RNC1 Compressor stream (23296 -> 33600 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 2044-2114 (bytes 523264-541440) hold a RNC1: Rob Northen RNC1 Compressor stream (18176 -> 16384 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 2116-2148 (bytes 541696-550144) hold a RNC1: Rob Northen RNC1 Compressor stream (8448 -> 16384 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2:chunks 2116-2148 (bytes 541696-550144)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 2150-2216 (bytes 550400-567552) hold a RNC1: Rob Northen RNC1 Compressor stream (17152 -> 16384 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 2218-2390 (bytes 567808-612096) hold a RNC1: Rob Northen RNC1 Compressor stream (44288 -> 16384 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 2392-2428 (bytes 612352-621824) hold a RNC1: Rob Northen RNC1 Compressor stream (9472 -> 27534 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 2430-2460 (bytes 622080-630016) hold a RNC1: Rob Northen RNC1 Compressor stream (7936 -> 16384 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 2462-2494 (bytes 630272-638720) hold a RNC1: Rob Northen RNC1 Compressor stream (8448 -> 28839 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 2496-2560 (bytes 638976-655616) hold a RNC1: Rob Northen RNC1 Compressor stream (16640 -> 25176 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 2562-2592 (bytes 655872-663808) hold a RNC1: Rob Northen RNC1 Compressor stream (7936 -> 25915 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 2594-2624 (bytes 664064-672000) hold a RNC1: Rob Northen RNC1 Compressor stream (7936 -> 24180 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 2626-2688 (bytes 672256-688384) hold a RNC1: Rob Northen RNC1 Compressor stream (16128 -> 25305 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 2690-2746 (bytes 688640-703232) hold a RNC1: Rob Northen RNC1 Compressor stream (14592 -> 25749 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 2748-2818 (bytes 703488-721664) hold a RNC1: Rob Northen RNC1 Compressor stream (18176 -> 17204 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 2820-2922 (bytes 721920-748288) hold a RNC1: Rob Northen RNC1 Compressor stream (26368 -> 17280 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 2924-2934 (bytes 748544-751360) hold a RNC1: Rob Northen RNC1 Compressor stream (2816 -> 10240 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 2936-2946 (bytes 751616-754432) hold a RNC1: Rob Northen RNC1 Compressor stream (2816 -> 8192 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 2948-2980 (bytes 754688-763136) hold a RNC1: Rob Northen RNC1 Compressor stream (8448 -> 8192 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 2982-3004 (bytes 763392-769280) hold a RNC1: Rob Northen RNC1 Compressor stream (5888 -> 8271 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 3006-3068 (bytes 769536-785664) hold a RNC1: Rob Northen RNC1 Compressor stream (16128 -> 4096 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 3070-3076 (bytes 785920-787712) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.79)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 3078-3082 (bytes 787968-789248) hold a RNC1: Rob Northen RNC1 Compressor stream (1280 -> 1920 bytes)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 3224-3230 (bytes 825344-827136) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.97)
- ChaosEngine_v1.2_AGA_1324/ChaosEngineAGA/Disk.2: chunks 3235-3239 (bytes 828160-829440) outside the filesystem hold data, not code (opcode density 0.017, entropy 6.59)
- 4 never-loaded region(s), 18432 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 34560 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### ChaosEngineAGA.Slave

- `return_minus_one_in_d0`: Loads the immediate value minus one into data register d0 and returns immediately to the caller.
- `slave_init_setup_and_jump`: Resets the stack pointer, stores the entry a0 pointer, calls two library vectors, writes a byte into game memory, runs three setup subroutines and jumps to a routine at offset 628.
- `save_regs_call_vector_or_alt`: Saves all registers on the stack, selects one of two parameter blocks based on a flag, calls a library vector through a2, restores registers and returns.
- `ciaa_pra_write_and_fire_button_check`: Writes the value 25 to CIAA port A at offset 3584 from a0, waits while bit 0 of the CIA-B port at an address is set, then rotates and inverts d0 and compares it with a byte at a program-counter-relative offset.
- `set_two_a5_offsets_to_99`: Stores the immediate value 99 into the two words at offsets 80 and 82 from a5, then returns to the caller.
- `relocate_offsets_in_a0_table`: Saves all registers, loads a base pointer from a program-counter-relative long, then walks a table at offset an address from the base, adding the base to each nonzero entry until a zero terminator, and restores registers.
- `dispatch_rnc_decompress_by_file_id`: Saves d1-d4/a0-a2, reads a byte from a table offset by the current file id, and if it is not the fourth entry and d2 is nonzero, calls the decompressor entry at offset 40 of the library base with computed d0/d1/d2 arguments, returning zero.
- `call_library_offset_24_preserving_all`: Pushes all data and address registers, loads the library base from the pc-relative pointer, calls the function at offset 24 of that base, then restores all registers and returns.
- `rnc_pack_method_unpack_to_buffer`: Checks the RNC signature, reads packed sizes, copies the bit stream backwards, then decodes literal and match runs into the caller's buffer using the library bit reader, returning the unpacked length.
- `decode_bitstream_against_mask`: Reads masked word pairs from a0 until the masked value equals the second word, then extracts two bit fields using d6 and d7 and returns the combined bits in d0.
- `mask_d0_and_consume_d7_budget`: Masks d0 with d6, subtracts d1 from d7, and if the budget went negative calls the bitstream refill routine before shifting d6 right by d1 and returning.
- `refill_bit_buffer_from_a3`: Adds d1 back to d7, shifts d6 right by d7, swaps d6, reads two bytes backwards from a3 into d6 with byte rotation, then resets d7 to sixteen minus d1.
- `read_four_bytes_into_d0`: Loads four bytes from the address in a0 into d0, shifting left by eight before each byte so the last byte read ends up in the low byte, then returns.
- `build_huffman_decode_tables`: Reads a code-length table from the stack, then fills a0 with per-symbol masks, bit-reversed codes, code lengths and shift counts for decoding, advancing d0 and d1 per pass.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `silence_all_four_audio_channels`: Clears two words at address 256, then points all four Paula audio channel pointers at address 256 with length one and returns, silencing playback.
- `wait_vblank_toggle_with_timeout`: Saves d0, waits while the word at absolute address 52f8 is non-negative, then repeatedly calls the routine at 12400 until it returns zero or the word at offset 11be of a5 becomes non-zero, restoring d0.
- `empty_rte_exception_stub`: Contains only an rte instruction, so it is an empty exception handler that immediately returns from the interrupt.
- `reset_custom_chips_and_palette`: Installs a level interrupt vector, silences audio and interrupt registers, clears sprite and color registers, then re-enables DMA and interrupts on the custom chip base.
- `call_audio_tick_if_flag_set`: Tests the longword at offset 2016 from a5; if it is nonzero, calls the external routine at the given absolute address, then returns.
- `call_audio_tick_if_byte_clear`: Tests the byte at offset 4524 from a5; if it is zero, calls the external routine at the given absolute address, then returns.
- `call_audio_tick_if_flag_set_alt`: Tests the longword at offset 2016 from a5; if nonzero, calls a different external routine at the given absolute address, then returns.
- `call_22dee_if_7e0_nonzero`: Tests the longword at offset 2016 from a5 and, when it is nonzero, calls the routine at absolute address 22dee before returning.
- `reset_display_and_restore_7e0`: Saves the status register and the longword at offset 2016 from a5, clears that slot, swaps display buffers, calls several library routines, sets byte 4524 from a5 to all ones, then restores the saved longword and returns.
- `swap_5920_longwords_between_a0_a1`: Exchanges 5921 longwords in blocks of four between the buffers pointed to by a0 and a1, advancing both pointers, then returns.
- `repeat_sound_command_37_times`: Saves d0-d1, calls a setup routine, then loops 38 times issuing sound commands with d0=3 and d1=37, calling two subroutines per iteration, and restores d0-d1.
- `send_sound_command_255_zero`: Saves d0-d1, sets d0 to 255 and d1 to zero, calls the sound command routine, then restores d0-d1 and returns.
- `advance_stream_pointer_and_send_word`: Loads a pointer from a5 offset, reads a word, and depending on its sign either sends a default command or sets a flag bit, sends commands, then stores the advanced pointer back.
- `advance_script_pointer_and_dispatch`: Loads the script pointer from a5 offset 4020, reads a word, and if negative checks a flag then sends command 64 or sets bit 7 of a status byte and dispatches commands 3 through the pc-relative jump table before storing the advanced pointer back.
- `spawn_effect_from_table_entry`: Saves registers, indexes a four-byte table entry by d1, validates its type and visibility bits, then sets up display pointers and issues several command calls to start the effect.
- `check_effect_slot_priority`: Reads the active effect mask at a5 offset 3918, tests bits 3 and 2, and compares the entry's byte 3 against the priority table at a5 offset 92 to decide whether to continue or abort.
- `install_weapon_slot_and_dispatch`: Compares the object's type byte against two active slots, then under interrupt mask stores its fields into per-slot tables, sets a bit in the active mask, loads a handler pointer into the slot and jumps to it.
- `resolve_secondary_weapon_frame`: For objects flagged with bit 5, indexes the pointer table at offset 1924, reads a frame count, calls the frame selector, then advances a0 past the chosen entry and loads two words into d1 and d2.
- `read_object_subtype_and_call`: Clears d2, copies the object's third byte into d2, tests bit 7 of the first byte and, if clear, calls the routine at offset 198 before branching onward.
- `restore_sr_and_registers_after_call`: Calls two nearby subroutines, moves d2 into d1, restores the status register from the stack, then pops d0-d4 and a0-a3 before returning.
- `advance_pseudo_random_seed_and_scale`: Updates the 16-bit seed at offset 3914 in a5 with a multiply-add step, then divides a fixed value by the scaled d1-d2 range and returns the result in d0.
- `reset_sprite_pointer_state`: Loads the fixed address 4c44 into both pointer fields at offsets fb0 and fb4 of a5, clears the long at offset 5c and the byte at f4e, then returns.
- `init_chaos_engine_audio_state`: Clears a5-relative state at offset 2360, computes several aligned pointers and stores them at offsets 1964, 1848, 1962, 2642, 2646, 2634, 2638, 2640, 2638, 2642, then calls two routines and initialises channel fields at offsets 2694 to 2707.
- `reset_audio_channel_parameters`: Stores 48 at a5+2712, clears words at a5+2720 and a5+2716, sets a5+2722 to 336, shifts the word at a5+2802 left by four into a5+2718, and writes 8496 and 8000 to a5+2006 and a5+2010.
- `stop_audio_and_install_vblank_handler`: Clears a5-relative bytes and words, sets a5+2708 to 2, loads the custom chip base into a3, installs a level 3 interrupt vector, enables DMA and interrupts, sets supervisor mode, stores 16 at a5+2014 and calls a routine.
- `setup_40000_byte_buffer_and_vblank`: Installs a vertical blank handler, sets a word at offset 2014 of a5 to 32, copies the base pointer at 2650(a5) into three buffer pointers, adds 40000 to one, sets a flag byte, and calls a system function with d0=2.
- `setup_64000_byte_buffer_aligned`: Installs a vertical blank handler, sets a word at offset 2014 of a5 to 16, copies the base pointer at 2650(a5) into a buffer pointer, adds 64000, rounds up to a multiple of 8, stores it in two pointers, sets a flag byte, and calls a system function with d0=2.
- `setup_8000_byte_buffer_and_vblank`: Installs a vertical blank handler, reads the pointer at 2650(a5) into d0, stores it in two buffer pointers, adds 8000 to d0 and stores it in a third, sets a flag byte, and calls a system function with d0=2.
- `swap_primary_and_secondary_pointer_slots`: Saves a0, swaps the two longword pointers at offsets 2666 and 2670 from a5, stores the new pointer at offset 2694 and sets the byte at offset 2711 to all ones before restoring a0.
- `swap_alternate_pointer_pair_and_flag`: Saves a0, swaps the two longword pointers at offsets 2680 and 2684 from a5, writes the swapped pointer to offset 2694 and sets the byte at offset 2711 to all ones, then restores a0.
- `setup_copper_list_and_enable_dma`: Saves all registers, checks the vertical blank bit, copies a pointer into the copper list, writes two 40000-step entries, sets COP1LCH and enables DMA with bit 15 set, then branches to a shared tail.
- `setup_copper_list_and_dma_for_8_entries`: Saves all registers, checks the vertical blank bit, copies display pointers, fills eight copper list entries with successive 8000-unit offsets, calls the copper setup routine and enables DMA.
- `vblank_coplist_and_audio_tick`: Interrupt handler saves all registers, copies copper list words from a5 offsets into the copper buffer, fills five blitter pointer entries, sets COP1LCH and DMACON, then calls audio and toggles a flag before returning.
- `per_frame_input_and_timer_update`: Writes the DMA control word, decrements a frame timer, reloads it from a table, updates a signed countdown, reads the joystick and mouse buttons and dispatches to the input handlers.
- `poll_flag_809_and_branch`: Tests bit 0 of the byte at offset 809 from a5 and branches to another routine if set, otherwise returns immediately.
- `update_two_axis_pairs_if_active`: Checks two pairs of words at offsets a3c/a3e and a40/a42 from a5; for each nonzero pair it calls two subroutines, then returns.
- `copy_sprite_block_rows_to_buffer`: Copies 16 words from two source buffers into two destination buffers, also copying words at offset 1056 into offset 132, then restores registers and returns.
- `advance_animation_frame_state`: Rotates three longword animation pointers at offsets 2634, 2638 and 2642 in a5, copies the previous pointer to 2694, mirrors three word values into 4260/4264/4268, swaps three longwords at 4028/4032/4036, derives a masked byte from 2782 and 4544 into 2698, sets 2699, sets the flag byte at 2711 and increments the word at 2724.
- `clear_5dc_longwords_at_7ac`: Saves a0 and d7 on the stack, loads the longword pointer from offset 7ac in a5 into a0, clears 1500 longwords starting there with a decrementing dbra loop, then restores d7 and a0 and returns.
- `setup_blitter_fill_with_a0_pointer`: Waits for the blitter busy bit in DMACONR to clear, enables DMA, programs BLTCON0, BLTAPTH, BLTAMOD, BLTDPTH and BLTDMOD with the a0 pointer and fixed values, writes the blit size and waits again before restoring registers.
- `setup_blitter_fill_with_fa14_height`: Waits for blitter idle, enables DMA, sets blitter pointers and modulos from a0, programs BLTSIZE with height value fa14, then waits for idle again and restores registers.
- `blitter_fill_1600x20_words`: Waits for blitter idle via DMACONR, programs BLTCON0, blitter pointers and modulos, sets BLTSIZE for a 1600 by 20 fill, waits again and returns.
- `blitter_fill_600x20_words`: Waits for blitter idle via DMACONR, programs BLTCON0, blitter pointers and modulos, sets BLTSIZE for a 600 by 20 fill, waits again and returns.
- `reset_sound_state_and_store_channel`: Saves registers, clears the words at offsets a3c and a40 from a5, calls the sound routine at 1d978, stores a1 at offset 938 from a5, sets the word at a40 to 16, restores registers and returns.
- `advance_sound_pointer_and_clear_channel`: Saves registers, sets d0 to 5, calls the sound routine at 1d978, adds 2 to a1, clears the word at a3e from a5, sets the word at a42 to 16, restores registers and returns.
- `set_sound_mode_and_clear_channel`: Writes 18 to the word at offset a3c from a5, clears the word at offset a40 from a5, then returns to the caller.
- `set_blitter_shift_and_clear_flag`: Writes the immediate value 18 to the word at offset a3e of the a5 frame and clears the word at offset a42, then returns.
- `blit_five_planes_with_blitter_wait`: Saves registers, programs blitter control, modulo and data pointers for five successive blits of 32000 bytes each, polling DMACONR bit 6 before each, then restores registers and returns.
- `blit_five_planes_then_call_sub_1120`: Programs the blitter for five successive plane blits with DMACONR waits, then copies a1 to a0, calls the routine at 1120, waits again and restores registers.
- `zero_filled_data_area`: This single zero word decodes as ori.b #0,d0; the bytes are zero-filled data referenced by address, not executable code.
- `init_audio_channels_and_voice_tables`: Scans 128 bytes at offset 952 from a0 for the highest value, then clears 31 voice structures, stores their pointers, silences the four audio volume registers, sets CIAA port bits and clears sound state bytes.
- `silence_audio_and_enable_dma`: Writes zero to the four audio volume registers and writes 15 to DMACON, enabling the audio DMA channels, then returns.
- `advance_audio_sequence_and_fade_voices`: Steps the current audio sequence pointer, programs four voice structures into the audio hardware registers, and applies a fade to each voice volume while DMA is enabled.
- `wait_for_raster_and_update_voices`: Polls the vertical position register until it changes, then calls the voice update routine for each of the four audio voice structures.
- `decode_audio_command_and_program_channel`: Reads a command word from the sequence data, looks up a sample descriptor, fills the channel structure fields, and writes AUDxLCH, AUDxLEN and AUDxVOL plus DMACON bits for the given channel.
- `apply_velocity_toward_target`: Reads the target distance at offset 22 of a4; if nonzero, adds or subtracts the speed byte at offset 21 to the position at offset 16 depending on the direction flag at offset 20, clamps to the target, clears the target and stores the position into 6(a5).
- `compute_sine_scaled_vertical_offset`: Uses the angle byte at 25(a4) to index a sine table, multiplies by the low nibble of 24(a4), shifts right by seven, negates if the angle sign bit is set, adds the result to the position at 16(a4) and stores it into 6(a5), then advances the angle.
- `dispatch_entity_command_nibble`: Reads the command word at 2(a4), switches on its low nibble to move the position at 16(a4) with clamping, call the velocity or sine routines, adjust the angle at 19(a4), toggle the CIAA fire bit, or set the global speed and direction bytes.
- `add_zero_immediate_to_d0_byte`: Adds the immediate value zero to the low byte of d0, leaving d0 unchanged, then falls through into the following code.
- `zero_word_data_area_entry`: A single zero word that decodes as ori.b #0,d0; it is zero-filled data referenced by address, not executable code.
- `zero_filled_data_table_31_words`: Thirty-one zero words that decode as ori.b #0,d0; this is a zero-filled data table referenced by address, not working code.
- `bitmask_data_table_bit0`: Seven data words whose bytes are not executable code; the final immediate sets bit 0, marking this as a bitmask table entry.
- `bitmask_data_table_bit1`: Seven data words of non-code bytes; the last immediate sets bit 1, identifying this table entry as the second bitmask value.
- `bitmask_data_table_bit2`: Seven data words of non-code bytes; the final immediate sets bit 2, identifying this table entry as the third bitmask value.
- `palette_black_fill_stub`: Writes eight zero bytes to d0 in sequence, ending with a small immediate, forming a data-like fill rather than executable logic.
- `blit_planar_tiles_to_bitplanes`: Saves registers, checks a dirty flag via a helper, then converts packed 4-bitplane tile data through a lookup table into two interleaved bitplane buffers at a1 and a2.
- `rebuild_palette_lookup_table`: If the palette index at offset 2096 differs from the cached one, stores it and fills 128 bytes at offset 2104 with interpolated values derived from the index.
- `decrement_timer_and_clear_on_expiry`: Subtracts sixteen from the word timer at offset 2096 in a5, and if it reaches zero or underflows, zeroes that timer plus the words at offsets 2620 and 2622.
- `increment_timer_and_clamp_to_max`: Adds sixteen to the word at offset 2096 in a5, clamps it to 256 and clears the words at offsets 2624 and 2626 when exceeded; trailing bytes are data, not executed.
- `run_level_setup_and_script_sequence`: Initialises display and sound buffers, runs scripted subroutine tables, loads level data, waits on timers and CIAA fire button, then restores audio and returns.
- `read_word_clear_flag_and_call_allocator`: Reads a word from the a6 stream into d0, loads a pointer from a5 offset, clears the byte flag at a5+4286, then calls the routine at a fixed address with a6 pushed and restores a6.
- `reset_audio_buffers_and_dispatch_table`: Frees three audio buffer pointers through the memory routine, stores a new buffer address, then indexes a table with a word from a6 and calls the dispatcher.
- `reset_state_flags_and_call_two_routines`: Calls the routine at pc offset 2348, reads a word from a6 into d0, clears byte at a5+120, sets byte at a5+4286 to 255, clears bit 0 at a5+4527, then calls the routine at pc offset 4488.
- `set_flags_and_call_two_helpers`: Calls two helper routines via pc-relative jsr, reads a word from the a6 pointer into d0, sets bytes at offsets 120 and 4286 of a5 to 255 and clears bit 0 at offset 4527 of a5.
- `clear_flag_wait_counter_and_call`: Clears byte at offset 120 of a5, sets bit 0 at offset 4527 of a5, calls a helper only if that bit was previously clear, then waits until the word at offset 2620 of a5 is zero and clears the byte again.
- `save_a6_call_two_routines_restore`: Pushes a6 onto the stack, calls two subroutines at pc-relative offsets, restores a6 from the stack and returns to the caller.
- `set_global_ff_and_call_supervisor`: Stores byte 255 at absolute address 846a, saves all registers, loads a pointer from pc-relative 13ca into a0, enters supervisor mode with sr 2700, calls the routine at 13ce, then restores sr and all registers and returns.
- `fetch_next_pointer_until_threshold`: Pushes a0, reads a 16-bit index from a table, increments that index in place, scales it by four, loads the pointed 32-bit value into d0, and returns early if it is below the value at offset 2086 in a5 or if bit 7 of byte 119 in a5 is set.
- `copy_826_a5_longword_to_a0`: Copies the longword at offset 826 from a5 into the address in a0, then branches to the code at 2632; it is a two-instruction fragment entered only by fallthrough.
- `call_audio_command_20_preserve_d0`: Saves d0 on the stack, loads the immediate value 20 into d0, calls the audio routine at a fixed address, then restores d0 and returns.
- `reset_byte_at_846a_if_nonzero`: Tests the byte at memory location 846a; if it is nonzero, writes the value 31 to that same byte, then returns.
- `render_text_string_to_bitmap_columns`: Parses a control-byte string in a0, handling cursor and color commands, and copies 16-row character glyphs from the font table into two bitmap planes at offsets 40000 apart.
- `fade_palette_entries_by_delta`: Saves all registers, computes a count from d0 and d2, copies the palette pointer at 2650(a5) into two work pointers, then repeatedly waits on the flag at 2711(a5) and advances the second pointer by 40 bytes until the count expires.
- `step_three_palette_components_down`: Loops nine times over the palette block at 857a6, subtracting 273 from each nonzero word at offsets 2, 6 and 10, calling the palette update routine after each group, then calls the palette reset routine.
- `reset_palette_pointers_and_colors`: Copies the base palette pointer at 2650(a5) into two work pointers, calls the palette setup routine three times with different pointers, then writes fixed color words into the palette block at 857a6.
- `draw_signed_offset_tile_column`: Saves all registers, computes a vertical tile offset from d0 sign and d2, then repeatedly advances a5+2694 by 40 bytes while polling the a5+2711 flag, calling one of two helpers based on d0 sign.
- `render_text_string_to_bitmap_columns_2`: Reads a length-prefixed string from a0, maps characters through a glyph table at a5+3460, and copies eight bytes per glyph into two bitmap planes at a5+2670 and 40000 bytes beyond, advancing 40 bytes per row.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `rebuild_level_tileset_and_palette`: When the a5+2711 flag is set, clears it, resets tile pointers and counters, waits for a5+2694 to match a5+2666, swaps the a5+2670 tile base, and rebuilds palette entries from a5+df8.
- `draw_text_string_with_line_spacing`: Walks a null-terminated string of character codes, skipping bytes equal to minus one, and for each other byte calls the glyph routine at the address in a6 with x in d6 and y in d7, advancing d6 by the word at offset 3464 of a5.
- `draw_glyph_without_advance`: Saves all registers, loads the glyph base pointer from offset 2662 of a5, copies d0 to d2 and d6/d7 to d0/d1, passes a2 as a1 and calls the glyph blitter routine, then restores registers and returns.
- `draw_glyph_advance_one`: Saves all registers, loads the glyph base pointer from offset 2662 of a5, copies d0 to d2 and increments d2 by one, moves d6/d7 to d0/d1, passes a2 as a1 and calls the glyph blitter routine before restoring registers.
- `copy_sprite_frame_to_screen_buffer`: Saves all registers, loads a pointer from offset 2670 of a5, increments d2 by one, moves d6 and d7 into d0 and d1, sets a1 to a2 and calls the blit routine, then restores registers and returns.
- `draw_tile_row_and_set_palette`: Sets d7 to 96, multiplies d2 by 4800 and adds it to a1, calls the tile routine, calls a library function, writes 40 to a global, sets d6 to 5, calls another library function with a4 and a6 saved, then returns.
- `decrement_index_and_blit_tile`: Decrements d2 and branches out if negative, otherwise multiplies d2 by 80 and adds it to a1, sets d7 to 8, calls the blit routine and branches back to the caller's loop.
- `advance_sprite_frame_and_trigger_sound`: Decrements d2, and if non-negative scales it by 160 to offset a1, sets d7 to 16, calls a helper, then calls two library routines and stores 8 in a global word before returning.
- `compute_tilemap_cell_pointer`: Multiplies d1 by 40 and adds twice the high nibble of d0 to a base pointer from a5, clears d3 to minus one and zeroes a global word, returning the computed offset.
- `copy_1249_longwords_to_buffer`: Copies 1249 longwords from an offset of a5 into the buffer pointer stored at 2670(a5), advancing both pointers, then returns.
- `copy_35c_longwords_to_40000_offset`: Copies 860 longwords from the a5-relative source table to the destination pointer at offset 2662(a5) plus 40000, then clears byte 120(a5), sets bit 0 at 4527(a5) and calls a routine if the bit was clear, waiting while word 2620(a5) is nonzero.
- `xor_d0_with_d19c_constant`: Exclusive-ORs the low longword of d0 with the immediate constant d19c and falls through to the following code without returning.
- `or_e6_and_9c_into_d0_bytes`: ORs the immediate byte e6 into the low byte of d0, then ORs the immediate byte 9c into the low byte of d0, and falls through to the next routine.
- `or_constant_bits_into_d0`: ORs the immediate bytes 126 and 156 into the low byte of d0, setting those bits in the register.
- `install_cia_interrupt_vectors_and_enable`: Sets the level 2 autovector, configures CIA-A port and timer registers, patches two vector slots with new addresses, and enables the vertical-blank interrupt in INTENA.
- `or_d0_into_d1_low_byte`: ORs the low byte of d0 into the low byte of d1, leaving the upper bytes of both registers unchanged.
- `cia_interrupt_dispatch_rte`: Reads CIAA interrupt control register into d0, acknowledges INTREQ bit 3, merges the pending bits into a stored mask and jumps through one of five handler vectors before returning with rte.
- `increment_frame_counter_call_vblank`: Saves all registers, loads the custom chip base into a3 and the game state base into a5, increments the frame counter at offset 2062 and calls the vertical blank routine.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `read_keyboard_events_update_joystick_bits`: Acknowledges INTREQ bit 3, reads a key code via the keyboard routine, sets or clears joystick direction bits in the state at offset 118 and stores the translated key code.
- `read_joystick_ports_update_direction_bits`: Reads CIAA fire buttons and both joystick direction words, calls the decode routine for each port, then sets direction and activity bits in the state block at a5.
- `decode_joystick_word_to_direction_bits`: Takes a joystick word in d0 and sets direction bits in d1 for up, down, left and right by testing bit patterns and an xor of shifted copies.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `pop_d5_d4_a0_and_return`: Pops a 16-bit value into d5, then a 16-bit value into d4, then a 32-bit value into a0 from the stack, and returns to the caller.
- `reset_game_state_and_dma`: Clears several a5-relative state bytes and words, sets a longword to all ones and another to an address, writes 33344 to DMACON at a fixed address, then calls cleanup routines on three pointers and four repeated subroutines before returning.
- `save_regs_call_1d978`: Pushes d0-d2, d7 and a1-a2 onto the stack, calls the subroutine at address an address, restores those registers and returns.
- `wait_for_byte_change_d0_times`: Saves d1, reads the byte at offset 2057 from a5 into d1, compares it with itself, and loops decrementing d0 until d0 expires, then restores d1 and returns.
- `wait_for_byte_change_at_a5_809`: Saves d1 on the stack, reads the byte at offset 809 from a5 into d1, compares it with itself and loops until it differs, then restores d1 and returns.
- `dispatch_command_word_to_handlers`: Saves d0, d2 and a1, stores the command word at offset 4562 from a5, masks the high bits and branches to per-command handlers that copy data, patch strings or load files, then returns.
- `queue_and_copy_request_block`: Repeatedly reads the request word and pointer from a5 offsets, searches a table of pending entries, copies the block into a free slot and calls the low-level transfer routine until it succeeds.
- `save_all_registers_call_17ce_restore`: Pushes all data and address registers onto the stack, calls the routine at 17ce, then pops all registers back and returns to the caller.
- `copy_title_mod_name_and_load_module`: Copies a null-terminated string into a buffer, then up to four times enables DMA, calls the module loader with the buffer, and on failure flashes COLOR00 red, green and yellow before branching away.
- `match_chaos1_name_against_table`: Stores a byte from the input, calls a comparison routine, then scans a table for a matching entry and compares bytes, returning zero on a match or minus one otherwise.
- `cycle_player_slot_until_match`: Reads the current player slot byte, saves it, then repeatedly calls the slot setup routine and compares the slot table entry against the reference table, advancing the slot with a mask until a match or wrap, returning -1 on failure.
- `load_slot_graphics_via_dma`: Saves registers, builds a source pointer from the slot byte, sets up dimensions and destination, enables DMA with a control word, calls the graphics loader, then restores DMA and registers.
- `select_and_load_slot_asset`: Saves registers, validates the current slot, looks up a table entry, checks a per-slot bit, computes a size, calls the asset loader and returns a status with the size in d1.
- `ori_b_zero_d3_padding_word`: Performs a bitwise OR of zero with the low byte of d3, leaving d3 unchanged; this single instruction is padding or data misdecoded as code.
- `retry_jsr_62e2_five_times_until_zero`: Saves d0-d2 and a0-a2 on the stack, calls the routine at address 62e2 up to five times, restoring registers each pass, and returns when the call reports zero or the counter expires.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_ascii_zero_then_add_d1_byte`: Writes the character zero into the byte at a0, then adds the low byte of d1 to that byte and advances a0, returning immediately.
- `index_jump_table_entry_by_d0`: Pushes d0, shifts it left twice to form a byte offset, loads the longword from the table at pc plus that offset into a1, moves a1 to d0, and if zero loads a fallback address from pc; restores d0 and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_byte_at_a0_offset_three`: Compares the byte at offset three from a0 with the value two and branches far away if equal, otherwise jumps to an external address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
