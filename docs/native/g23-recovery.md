# Game 23 (WWFEuropeanRampageTour v1.0 1298): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00023). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4383661 top-level bytes in 47 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 31 of 31 units identical (bytes and relocations), 133 raw-word lines |
| Functions named and described | pass, 287 of 287 named (0 author symbols), 287 of 287 described (19 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 5383 of 5390 (99.87%) |
| Recovered without load address (counted in proof 1, flagged) | 18303 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8081 of 8093 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 218249 |
| code | 27928 |
| data | 4133978 |
| padding | 2 |
| unused | 3504 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 228 | 28 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 1332 | 29 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 838 | 18 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 595 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 435 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 143 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 196 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 123 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 128 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 53 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2601 | 115 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 527 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 838 | 18 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 5 |
| Disk.1:chunks 1884-1950 (bytes 482304-499456)#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- WWFEuropeanRampageTour_v1.0_1298/WWFEuropeanRampageTour_v1.0_1298_hdf/WWFEuropeanRampageTour/Disk.1: raw disk image (958464 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- WWFEuropeanRampageTour_v1.0_1298/WWFEuropeanRampageTour_v1.0_1298_hdf/WWFEuropeanRampageTour/Disk.2: raw disk image (970752 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- WWFEuropeanRampageTour_v1.0_1298/WWFEuropeanRampageTour_v1.0_1298.hdf: hard-disk image (AmigaDOS OFS, 4672 blocks): its filesystem parses and all 8 files in it are byte-identical to the extracted tree WWFEuropeanRampageTour_v1.0_1298_hdf, which is recovered file by file; no block outside the filesystem holds data
- 7 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 321 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m3000): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m3000): 588 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 252 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 48 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 76 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 46 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 79 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 39 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2067 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 472 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 326 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (55 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- WWFEuropeanRampageTour_v1.0_1298/WWFEuropeanRampageTour_v1.0_1298_hdf/WWFEuropeanRampageTour/Disk.1: 6 logged loads cover 481824 bytes of the image
- WWFEuropeanRampageTour_v1.0_1298/WWFEuropeanRampageTour_v1.0_1298_hdf/WWFEuropeanRampageTour/Disk.1: chunks 1-1 (bytes 256-512) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.07)
- WWFEuropeanRampageTour_v1.0_1298/WWFEuropeanRampageTour_v1.0_1298_hdf/WWFEuropeanRampageTour/Disk.1: chunks 1884-1950 (bytes 482304-499456) hold a RNC1: Rob Northen RNC1 Compressor stream (17152 -> 18303 bytes)
- WWFEuropeanRampageTour_v1.0_1298/WWFEuropeanRampageTour_v1.0_1298_hdf/WWFEuropeanRampageTour/Disk.1:chunks 1884-1950 (bytes 482304-499456)#unpacked: high-entropy data file (7.66 bits/byte), classified as packed data
- WWFEuropeanRampageTour_v1.0_1298/WWFEuropeanRampageTour_v1.0_1298_hdf/WWFEuropeanRampageTour/Disk.1:chunks 1884-1950 (bytes 482304-499456)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- WWFEuropeanRampageTour_v1.0_1298/WWFEuropeanRampageTour_v1.0_1298_hdf/WWFEuropeanRampageTour/Disk.1: chunks 1952-2074 (bytes 499712-531200) hold a RNC1: Rob Northen RNC1 Compressor stream (31488 -> 37938 bytes)
- WWFEuropeanRampageTour_v1.0_1298/WWFEuropeanRampageTour_v1.0_1298_hdf/WWFEuropeanRampageTour/Disk.1: chunks 2076-2494 (bytes 531456-638720) hold a RNC1: Rob Northen RNC1 Compressor stream (107264 -> 126272 bytes)
- WWFEuropeanRampageTour_v1.0_1298/WWFEuropeanRampageTour_v1.0_1298_hdf/WWFEuropeanRampageTour/Disk.1: chunks 2496-2846 (bytes 638976-728832) hold a RNC1: Rob Northen RNC1 Compressor stream (89856 -> 111630 bytes)
- WWFEuropeanRampageTour_v1.0_1298/WWFEuropeanRampageTour_v1.0_1298_hdf/WWFEuropeanRampageTour/Disk.1: chunks 2848-3743 (bytes 729088-958464) hold a RNC1: Rob Northen RNC1 Compressor stream (229376 -> 28886 bytes)
- WWFEuropeanRampageTour_v1.0_1298/WWFEuropeanRampageTour_v1.0_1298_hdf/WWFEuropeanRampageTour/Disk.2: 1 logged loads cover 48 bytes of the image
- WWFEuropeanRampageTour_v1.0_1298/WWFEuropeanRampageTour_v1.0_1298_hdf/WWFEuropeanRampageTour/Disk.2: chunks 1-398 (bytes 256-102144) hold 2 packed streams (RNC1: Rob Northen RNC1 Compressor) found by scanning, 99% of the bytes
- WWFEuropeanRampageTour_v1.0_1298/WWFEuropeanRampageTour_v1.0_1298_hdf/WWFEuropeanRampageTour/Disk.2: chunks 400-760 (bytes 102400-194816) hold a RNC1: Rob Northen RNC1 Compressor stream (92416 -> 84480 bytes)
- WWFEuropeanRampageTour_v1.0_1298/WWFEuropeanRampageTour_v1.0_1298_hdf/WWFEuropeanRampageTour/Disk.2: chunks 762-944 (bytes 195072-241920) hold a RNC1: Rob Northen RNC1 Compressor stream (46848 -> 84480 bytes)
- WWFEuropeanRampageTour_v1.0_1298/WWFEuropeanRampageTour_v1.0_1298_hdf/WWFEuropeanRampageTour/Disk.2: chunks 946-1128 (bytes 242176-289024) hold a RNC1: Rob Northen RNC1 Compressor stream (46848 -> 84480 bytes)
- WWFEuropeanRampageTour_v1.0_1298/WWFEuropeanRampageTour_v1.0_1298_hdf/WWFEuropeanRampageTour/Disk.2: chunks 1130-1806 (bytes 289280-462592) hold a RNC1: Rob Northen RNC1 Compressor stream (173312 -> 84480 bytes)
- WWFEuropeanRampageTour_v1.0_1298/WWFEuropeanRampageTour_v1.0_1298_hdf/WWFEuropeanRampageTour/Disk.2: chunks 1808-2126 (bytes 462848-544512) hold a RNC1: Rob Northen RNC1 Compressor stream (81664 -> 84480 bytes)
- WWFEuropeanRampageTour_v1.0_1298/WWFEuropeanRampageTour_v1.0_1298_hdf/WWFEuropeanRampageTour/Disk.2: chunks 2128-2212 (bytes 544768-566528) hold a RNC1: Rob Northen RNC1 Compressor stream (21760 -> 41600 bytes)
- WWFEuropeanRampageTour_v1.0_1298/WWFEuropeanRampageTour_v1.0_1298_hdf/WWFEuropeanRampageTour/Disk.2: chunks 2214-2372 (bytes 566784-607488) hold a RNC1: Rob Northen RNC1 Compressor stream (40704 -> 41600 bytes)
- WWFEuropeanRampageTour_v1.0_1298/WWFEuropeanRampageTour_v1.0_1298_hdf/WWFEuropeanRampageTour/Disk.2: chunks 2374-3791 (bytes 607744-970752) hold a RNC1: Rob Northen RNC1 Compressor stream (363008 -> 41600 bytes)
- 1 never-loaded region(s), 18303 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `disk_track_read_retry_loop` (Disk.1 +an address loaded at an address (270848 bytes)): Sets up CIA and custom chip registers for disk DMA, then reads a track's sectors from the buffer at a0, retrying on errors and toggling the disk select line via CIAB. Kind: disk track read retry loop with timing/error sensitivity. Pass path: the branch returning CONCAT22(uVar1,in_D0w) where in_D0w >= 0.
- `disk_track_read_and_decode` (runtime segment an address (play-rs-m1500)): Sets up CIAB and custom chip registers for disk DMA, waits on CIAB status bits, reads track data through a0 and decodes sector bytes into d5-d7 and d0 while toggling CIAB control bits. Kind: disk track/sector validation and checksum/CRC check. Pass path: the branch where in_D0w >= 0 (the loop that processes data and eventually reaches u25__l0_3234 with a non-negative result).

## Functions


### game.slave

- `return_minus_one_at_entry`: Loads the immediate value minus one into d0 and returns immediately, providing a constant return value at program entry.
- `store_a0_and_check_rom_checksum`: Saves a0 to a global, loads a base pointer into a5 and stores it at absolute address four, calls a checksum routine, compares d0 with a magic value, then sets up a callback and jumps through a5.
- `verify_rom_then_call_callback`: Saves d0-d1 and a0-a2, checksums memory at a fixed address, compares the result with a magic value, invokes a callback through a2, restores registers, loads a word from offset forty of a6 into d2 and returns.
- `dispatch_by_loaded_overlay_id`: Saves d0-d1/a0-a2, calls the overlay loader with identifier a fixed address, then compares the returned word against two known overlay ids and jumps through the selected handler table entry.
- `install_trampoline_and_call_library`: Writes a jump instruction and target address into the buffer at a0, calls library offset 32 via a2, restores registers, then jumps through the new trampoline and returns the loaded a0.
- `clear_intreq_bit_and_rte`: Writes 8192 to the INTREQ hardware register to acknowledge an interrupt, executes four no-ops for timing, and returns from the exception with rte.
- `store_incremented_byte_to_global`: Pushes a0, increments the low byte of d0, and if the result is nonzero stores that byte to a program-counter-relative global, then restores a0 and returns.
- `call_library_offset_and_return_handle`: Saves d0-d1 and a0-a2, loads a program-counter-relative string into a0, calls offset 100 of the library base at a program-counter-relative pointer, restores registers, returns the call result in d0 and 58 in d1.
- `seed_register_rotation_constants`: Loads a fixed 32-bit constant into d0, stores it to absolute address 128, then copies rotated variants of it into d5, d2, d3, d4, d6 and d7 before returning.
- `clear_byte_and_rotate_d0`: Clears the byte at offset 3072 from a0, complements all bits of d0, rotates d0 right by one bit, then branches back to an earlier address.
- `toggle_global_flag_bits`: Saves a0 on the stack, loads a nearby program-relative address into a0, exclusive-ORs the byte there with 3, restores a0 and returns.
- `compute_screen_offset_and_call_blit`: Uses d0 as an index into a coordinate table, computes a byte offset from the table entries, adds it to a0, calls a library routine through a2, and returns the adjusted a0.
- `open_two_libraries_and_close_first`: Saves d1 and a0-a2, opens two libraries via the exec library vector at offset 40 using names at two pc-relative addresses, then closes the first library via vector offset 48 and restores registers.
- `close_exec_library_via_vector`: Saves d1 and a0-a2, loads the exec library base from a pc-relative pointer, calls its close-library vector at offset 48, then restores registers and returns.
- `wait_for_vertical_position_change`: Saves d0-d1, reads the VHPOSR hardware register repeatedly up to seven times until its value changes, then restores registers and returns.
- `write_d1_to_a0_offset_58`: Stores the low word of d1 into memory at offset 88 from a0, then branches to a wait loop that polls bit 6 of the byte at offset 2 of a0.
- `write_d3_to_a0_offset_58`: Stores the low word of d3 into memory at offset 88 from a0, then branches to a wait loop that polls bit 6 of the byte at offset 2 of a0.
- `store_a3_and_d0_into_a5_struct`: Writes the long value of a3 to offset 84 from a5 and the low word of d0 to offset 88 from a5, then branches to a later address.
- `store_a1_and_d3_into_a5_struct`: Writes the longword in a1 to offset 84 of the a5 structure and the word in d3 to offset 88, then branches onward.
- `write_audio_period_and_wait_busy`: Stores the word value 13322 at offset 88 of the a0 structure, then waits while bit 6 of the byte at offset 2 stays set.
- `or_d2_into_d3_store_word`: Combines d2 into d3 with a word OR, stores the result at offset 88 of the a5 structure, then branches onward.
- `copy_long_and_two_words_advance_a0`: Copies a longword from (a0) to (a1), then a word from (a0) to (a4) and another word to (a3), advancing a0 past all three, then branches onward.
- `store_d0_word_and_clear_a6_field`: Stores the low word of d0 at offset 88 of a0, calls a nearby routine, clears the longword at offset 238 of a6, and returns.
- `wait_for_bit6_clear_in_status_word`: Tests bit 6 of the word at offset 2 of a0 (or a5 on the second path), branching back while it is set and returning once it clears.
- `call_library_offset_24_preserving_regs`: Saves d0-d1 and a0-a2 on the stack, loads the library base from a pc-relative longword, calls the routine at offset 24 of it, then restores the registers and returns.
- `call_library_offset_32_preserving_regs`: Saves d0-d1 and a0-a2 on the stack, loads the library base from a pc-relative longword, calls the routine at offset 32 of it, then restores the registers and returns.
- `library_base_stub_ori_pea_branch`: Clears the low byte of d0 with an ori, pushes the longword at address minus one as a pea, then branches forward to the following code; the bytes are a small stub, not a loop.
- `push_two_pea_values_and_return`: Pushes the immediate values 5 and 9 onto the stack via pea, adjusts the stack pointer by four, then returns to the caller.

### logged disk load from Disk.1

- `main_game_loop_setup_and_frame_sequencer`: Sets supervisor stack, initializes copper and bitplanes, then repeatedly runs the frame sequencer, waits on the vertical position register, and advances the scripted animation index.
- `init_custom_chips_and_copper_list`: Silences interrupts and DMA, configures CIAA and CIAB ports, installs vertical blank and copper interrupt vectors, sets display and copper registers, copies the copper list to chip memory and enables DMA.
- `ciaa_fire_button_toggle_keymap_bit`: Interrupt handler saves a0/d0, acknowledges INTREQ bit 3, reads CIAA_PRA fire button, toggles a byte in the keymap table indexed by the inverted rotated button value, then restores registers and returns from exception.
- `vblank_read_joystick_and_fire_edges`: Vertical blank interrupt saves all registers, increments a frame counter, calls the copper/audio update, builds a button bitmask from joystick direction bytes and CIAA_PRA fire bits, stores newly pressed edges, calls the game tick, restores registers and returns from exception.
- `scale_32_words_by_volume_nibbles`: Reads a signed volume step from a6 offset 1, advances it toward zero, then scales 32 words from the source pointer into the destination pointer by multiplying each nibble, optionally inverting the result when a6 offset 39 is set.
- `scramble_d0_bit_permutation`: Masks d0 with 771, derives a shifted copy in d1, XORs them, then rotates and add-extends bytes to produce a permuted result in d0 and d1 before returning.
- `setup_a1_from_pc_relative_table`: Loads a PC-relative table into a0, scales d0 by eight, adds the table entry at that offset to a1, stores a1 at offset 14 of a6, and advances a0 by the table word.
- `decode_rnc_packed_block_to_buffer`: Saves all registers, allocates 384 bytes on the stack, verifies the RNC signature, unpacks the compressed block into the destination buffer using bit-stream reads, then restores registers and returns.
- `match_bitmasked_pair_and_extract_bits`: Scans word pairs at a0 until the masked first word equals the second, then reads two byte fields and returns a bit value built from d6 shifted by the field counts, calling the refill routine when d7 runs low.
- `mask_d0_with_d6_and_refill_bits`: Masks d0 with d6, subtracts d1 from d7 and, if the bit budget went negative, calls the refill routine before shifting d6 right by d1 and returning the masked value.
- `refill_bit_buffer_from_a3_stream`: Adds d1 back into d7, shifts d6 right by d7, swaps d6, advances a3 by four and loads two bytes backwards into d6 with a byte swap, then resets d7 to sixteen minus d1.
- `read_four_bytes_into_d0`: Loads four bytes from the address in a0 into the low bytes of d0, advancing a0 by four, then returns.
- `build_huffman_decode_tables`: Reads a code-length table via a helper, then fills a0 with per-symbol masks, bit patterns, lengths and offsets for decoding, returning after all lengths are processed.
- `dispatch_compressed_block_decoder`: Uses d0 as an index into tables at fixed addresses to compute a0 and a1 pointers, then branches to the shared decompression entry point.
- `write_f0_and_wait_for_clear`: Writes the byte value 240 to offset 1 of the a6 structure, then spins reading that same byte until it becomes zero before returning.
- `set_two_control_bytes`: Writes the byte value 16 to offset 1 of the a6 structure and the byte value 1 to offset 0, then returns to the caller.
- `copy_ten_thousand_longwords`: Copies 10000 longwords from the source address in a0 to the destination address in a1, incrementing both pointers, then returns.
- `blit_sprite_to_two_buffers`: Looks up a word offset from a table using d1, adds d0 twice, then programs the blitter with modulo, size and pointers to copy a sprite from one buffer to two destinations, waiting on blitter busy between four chunks.
- `blit_sprite_to_single_buffer`: Computes a table-based offset from d1 and d0, sets blitter modulo and size registers, then copies a sprite from the a0 source into one destination buffer in four chunks while polling the blitter busy bit.
- `xor_decompress_into_buffer`: Decodes a run-length stream from a2 into a1, using a3 as XOR key data, copying or XORing words and longwords until the output reaches the end pointer, then returns.
- `swap_copper_list_pointer_pair`: Swaps the two longwords at offsets 2 and 6 of the a6 structure, then writes the new value and four values incremented by 8000 into the copper list at a0.
- `draw_four_line_edges`: Saves d0,d1,d2,d3 into d6,d7,a3,a4, then calls the line routine four times to draw the four edges of a rectangle between the given coordinates.
- `blitter_fill_rect_with_pattern`: Computes a rectangle from d0-d3, builds blitter control words and modulo values, then programs the blitter registers to fill five successive screen rows with the pattern data.
- `disk_track_read_retry_loop`: Sets up CIA and custom chip registers for disk DMA, then reads a track's sectors from the buffer at a0, retrying on errors and toggling the disk select line via CIAB.
- `disk_select_pulse_delay`: Clears then sets bit 0 of CIAB port A to pulse the disk select line, inserts two nops for timing, and loads 2404 into d0 before falling through.
- `write_disk_sync_and_wait_ready`: Writes the low byte of d0 to CIAA offset 1536 and the high byte to offset 1792, then waits until bit 1 of CIAA offset 3328 is set before returning.
- `wait_blitter_done_and_store_status`: Stores a1 into offset 32 of a5, writes 38940 twice to the custom chip register at offset 36, polls bit 1 of offset 30 until set, saves it to offset 156, then writes 16384 to offset 36.
- `decode_planar_bitmap_to_chunky`: Checks for a sync word, skips to a plane offset scaled by d0, then converts 128 pairs of bitplane longs into chunky pixels at a0 using a a fixed address mask and accumulates a checksum in d1.
- `relocate_offset_table_entries`: Saves a0, calls a routine with a pc-relative table, restores a0, then adds a0 to each of 59 word-indexed long entries in a pc-relative offset table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_to_audio_init_body`: Executes an unconditional branch to a target further ahead in the same segment, transferring control without touching any registers or memory.
- `branch_to_audio_init_alternate`: Executes an unconditional branch to a nearby target in the same segment, transferring control without modifying registers or memory.
- `init_audio_channels_and_interrupts`: Scans a table to pick a free audio slot, clears thirty channel structures, stores pointers, then programs CIAA, interrupt and DMA registers to enable audio playback.
- `init_four_audio_channels_and_volume`: Points a5 at each of the four audio channel register blocks and calls the channel setup routine with a parameter block, then loads the volume table and sets the master volume word.
- `copy_sound_entry_and_check_type`: Copies the longword at a0 plus d1 into the sound entry at a6, reads the type byte, and if it equals fifteen stores the following byte to the effect flag and clears the next flag.
- `decode_and_start_sound_entry`: Fetches a sound entry, decodes its type and volume, fills the channel structure from the sample table, sets the audio period and DMA mask, and starts playback.
- `ciab_timer_interrupt_disable_dma_and_ack`: Saves d0, stores a pointer at address 120, reads CIAB interrupt control, writes 2 to CIAB timer high, sets DMAEN with bit 15 in DMACON, restores d0, acknowledges INTREQ bit 13 and returns from interrupt.
- `ciab_interrupt_ack_and_return`: Saves d0-d1 and a0/a5-a6, reads the CIAB interrupt control register into d0, calls the sound service routine, restores the registers, acknowledges interrupt bit 13 in INTREQ and returns from the exception.
- `set_audio_periods_and_volume_step`: Points a5 at the custom chip base, copies four audio channel pointer and length pairs from tables into AUD0-AUD3 registers, then adjusts the volume step counter and channel index in the sound state bytes.
- `dispatch_entity_action_by_low_nibble`: Calls a setup routine, reads the action code from the low nibble of the word at offset 2 of a6, and branches to the matching action handler, copying a6 offset 16 to a5 offset 6 first.
- `select_and_apply_entity_movement_step`: Copies a6 offset 16 to a5 offset 6, then either picks a table entry by direction nibble or adds/subtracts a masked step from the position word at a6 offset 16, clamping it between 113 and 856.
- `find_nearest_waypoint_and_set_target`: Scans a 74-byte waypoint table for the first entry not below the position at a6 offset 0, stores the chosen waypoint at a6 offset 24, and sets the direction flag at a6 offset 22.
- `update_vertical_velocity_and_clamp`: Reads the vertical speed and direction fields at offsets 24 and 22 of the object, adds or subtracts the speed byte at offset 23 from the position at offset 16, clamps the position, then looks up a table entry and stores the result at offset 6 of a5.
- `update_horizontal_velocity_and_angle`: Computes a horizontal velocity from the angle byte at offset 27 and the speed nibble at offset 26 using a sine table, adds it to the position at offset 16, stores the result at offset 6 of a5 and advances the angle.
- `dispatch_control_command_nibbles`: Calls a setup routine, then dispatches on the low nibble of the command byte at offset 2 and the high nibble of the byte at offset 3, handling joystick fire bit writes to CIAA and updating object fields.
- `sound_command_dispatch_by_nibble`: Reads the low nibble of the command byte at offset 3 of the sound structure and branches to the matching sound command, handling volume, period, DMA enable and channel selection through a5 and the sound state bytes.
- `advance_sprite_frame_and_flip_pixel`: Uses the high nibble of byte 31(a6) to index a table, adds the value to 35(a6), and when bit 7 sets, wraps the pointer at 36(a6) and inverts the byte it points to.
- `inline_data_table_after_code`: These two words are data bytes reached by a pc-relative lea, not executable code; they are read as a table by callers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `play_runtime_init_sequence`: Calls six unnamed routines and writes the custom chip registers BLTAFWM, BLTCON0, DMACON, INTENA, INTREQ and VHPOSR to set up the play runtime.
- `init_display_and_cia_registers`: Configures CIA and custom chip registers, installs two interrupt vectors, copies 59 longwords to a buffer, sets bitplane and copper registers, then returns.
- `vertical_blank_disk_interrupt`: Saves registers, acknowledges INTREQ bit 3, checks CIAA_PRA disk status, toggles a byte in a table indexed by d0, restores registers and returns from interrupt.
- `read_joystick_buttons_and_dispatch`: Saves all registers, acknowledges CIA interrupt, increments a counter, calls the input scan, builds a button bitmask from five flag bytes, XORs it into previous state, reads JOY0DAT and JOY1DAT through a scramble routine, then jumps through the level vector and returns with rte.
- `scale_palette_by_fade_level`: Reads a signed fade step from offset 1 of a6, adjusts it, then scales each of 32 colour words from the source pointer by that factor and writes the result to the destination pointer.
- `scramble_joystick_delta_bits`: Masks d0 with 771, mixes bits through shifts, rotates and add-with-extend to produce a scrambled value in d0 used to encode joystick movement deltas.
- `blit_sprite_rows_to_two_buffers`: Looks up a per-index offset table, computes source and destination pointers, programs four chained blitter copies of 8000-byte rows into two buffers, then jumps to a fixed address.
- `blit_sprite_rows_to_single_buffer`: Looks up a per-index offset table, computes a destination pointer, programs four chained blitter copies of 8000-byte rows from a0 into one buffer, then jumps to a fixed address.
- `draw_four_edges_between_corners`: Saves d0, d1, d2 and d3 in d6, d7, a3 and a4, then calls a line routine four times to draw the edges between the two corner points.
- `blit_span_line_with_modulo`: Sets up custom chip blitter registers from d0-d3 coordinates and a table at a1, then starts repeated blits advancing a2 by 8000 bytes each time until the blitter busy bit clears.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `save_regs_and_run_game_init`: Stores d1-d5 into the a6 frame, clears a 218-byte buffer, calls several setup routines, runs two indirect code pointers, and then loops forever.
- `delayed_state_transition_timer`: Checks a6 frame flags and a countdown at offset 38; when it expires, waits on a byte, clears a 218-byte buffer, calls setup routines and jumps into the main loop.
- `dispatch_indexed_jump_table`: Reads a word from the structure at a6, doubles it, adds the offset from the table after the current instruction, and jumps to the resulting address.
- `copy_indexed_word_and_clear_target`: Loads a pointer from offset 46 of a6, copies an indexed word from a6 into its offset 14 field, then clears offset 14 of the pointer at offset 92.
- `copy_two_indexed_words_to_globals`: Copies two words indexed by the values at offsets 2 and 4 of a6 into two fixed global memory locations, then returns.
- `check_zone_bounds_negative_flag`: Reads word at a0 and, when the zero flag is set, returns -1 if it is negative and below the threshold, else 0; otherwise checks x and y bounds at offsets 0 and 8 and returns -1 or 0 in d0.
- `check_zone_bounds_positive_side`: Reads word at a0; if negative, verifies it is not below the lower limit and that the word at offset 8 lies between the two bounds, returning -1 or 0 in d0.
- `check_x_at_least_123`: Loads the word at a0 into d0, branches away if it is negative or less than 123, otherwise sets d0 to -1 and returns.
- `check_entity_offscreen_vertical_band`: Reads word fields at offsets 0, 8 and 16 of the structure pointed to by a0 and returns -1 only when the entity is active, within the vertical band and horizontally close enough; otherwise returns 0.
- `check_entity_alternate_band_test`: Reads the same a0 word fields but returns -1 when the entity is inactive or outside the alternate vertical band, and 0 when it is active and inside the band.
- `build_four_sprite_descriptors_from_table`: Clears a block of bytes, then copies four records from a source table into a destination buffer, filling in sprite pointers, dimensions and per-entry table values before storing the resulting pointers into globals and the a6 structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `unidentified_13_instruction_block`: A thirteen-instruction block entered from outside static flow; the decompiler timed out, so only its instruction count and external entry are known, not its register or memory effects.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `play_rs_m1500_entry_sequence`: Runs a 25-instruction sequence in the play-rs-m1500 runtime segment, entered from outside static flow during the reference run.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_word_and_increment_counter`: Branches on the prior condition, reads a word from a0, negates it if negative, compares it to 96, and increments a byte counter at offset 30, returning 232 if it reaches 25.
- `increment_attack_counter_and_reset_state`: Reads a signed word from a0, takes its absolute value, and for state 70 or 21 increments a byte counter at offset 30 or 15; when the counter reaches 25 it resets state fields and loads animation pointers from a table.
- `set_attack_pose_for_direction`: Checks the sign of the word at a0 shifted right by four; for direction -8 or 7 with matching target fields it writes pose values -120 or 114 into a0 and sets state 63.
- `set_attack_pose_and_advance_state`: Skips if byte at offset 37 is 8; for direction -8 or 7 with allowed target fields it writes pose values -120 or 114, sets state 21, then advances the state machine and reloads animation pointers.
- `compare_offsets_and_swap_pointers`: Loads byte at offset 31 from a0 into d0, compares it with the byte at offset 31 from a1, and if a0's is not lower swaps a1 and a0, clears byte at offset 66 of a6, loads 704 into d0 and 712 into d1, then branches away.
- `decrement_counter_or_advance_index`: Tests d0; if zero returns, otherwise decrements d0 and if nonzero branches away; else increments byte at offset 31 of a0, compares it with 3 and branches away if below 3.
- `play_rs_m1500_secondary_sequence`: Executes a 22-instruction sequence in the play-rs-m1500 runtime segment, called by one unnamed routine and entered from outside static flow.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `select_opponent_and_swap_x_positions`: Checks the opponent's byte at offset 31; if above 2, clears the active flag, stores the opponent pointer at offset 58, and swaps the two objects' word values at offset 8.
- `decrement_timer_and_advance_attack_phase`: Decrements the word timer at offset 68; when it expires or an attack byte at offset 30 equals 7, resets both objects' attack bytes and increments the phase byte at offset 31.
- `relink_object_after_attack_animation`: For objects with word at offset 8 zero and word at offset 4 between 44 and 64, relinks the object from its offset 46 pointer, adjusts position and offset 4, and patches pointer fields.
- `reset_animation_pointers_and_counter`: Tests the word at offset 12 of the a0 structure; if nonzero loads the address of a nearby table into a1, then stores a1 at offsets 96 and 100 of a0 and clears the word at offset 106.
- `scale_duration_by_ratio`: Sets the word at offset 30 of a0 to the maximum, then if the word at offset 50 of a0 is nonzero computes a scaled value from a1's offsets 54 and 50 divided by a0's offset 50 and stores it at offset 30.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `input_axis_and_button_dispatch`: Calls several input polling routines, inspects the returned direction and button bits, adjusts the word at offset 8 of a0, and updates bytes at offsets 31, 66 and 67 of a6 based on the results.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `advance_animation_frame_or_clear_timer`: Increments the byte at offset 30 of the structure pointed to by a0, clears the long at offset 84 when the frame count exceeds 4 or a flag is set, and tests bits of d0 against the word at offset 12.
- `thirty_seven_instruction_runtime_block`: A 37-instruction block in the play runtime segment entered from outside static flow; the decompiler timed out, so only its size and entry context are known.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `twenty_two_instruction_runtime_block`: A 22-instruction block in the play runtime segment entered from outside static flow; no decompiled function was produced, so only its size and entry context are known.
- `adjust_actor_position_by_flags`: Reads a signed word at a0 and its offsets, tests bits of d0, and conditionally adds 3 or subtracts 16 from the word at a0 and 6 from the word at offset 25, clamping that to zero.
- `clear_actor_field_if_active`: If the byte at offset 31 of a0 is nonzero and bit 3 of d0 is clear, writes zero to the longword at offset 84 of a0, then returns.
- `start_round_timer_and_clear_flags`: When the byte at offset 66 of a6 is zero and the word at offset 70 is nonnegative, sets timer bytes and word, clears bytes at offset 32 of a0 and a1, and stores a pointer at offset 62 of a6.
- `start_round_timer_and_clear_players`: Checks byte 66(a6) and word 70(a6); if idle and non-negative, sets state bytes, timer 200, clears byte 32(a0) and 32(a1), sets word 70(a6) to 2 and stores a code pointer at 62(a6).
- `clear_player_54_if_not_protected`: If word 10(a1) equals 80 and bit 7 of byte 14(a0) is set, returns; otherwise if byte 21(a1) is zero, clears long 84(a0) and returns.
- `decrement_player_health_on_hit`: When bit 7 of d1 is set, sets word 20(a6) to 1 and, if byte 72(a6) is below 4, subtracts word 60(a0) from word 30(a0); if negative, resets byte 72(a6), clears bit 0 of a hardware byte and reduces word 0(a0).

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_timer_and_reset_state`: If bit 7 of d1 is set, stores 1 at a6 offset 20; when the byte at a6 offset 72 is below 4, subtracts the word at a0 offset 52 from the word at a0 offset 30, and if negative clears that byte, clamps the word at a0 offset 8 to zero and returns 224 in d0.
- `advance_state_above_70_and_load_anim`: When the word at a0 offset 16 is nonzero, moves the word at a0 offset 8 to 71 if it is at least 70, otherwise calls a helper and clamps it to 16; then clears bytes at offsets 19 and 20, sets the word at offset 10 to 632, loads pointers at offsets 96 and 100 from a table indexed by the word at offset 12, and zeroes offsets 30, 106 and 20.
- `rewind_state_below_22_and_load_anim`: When the word at a0 offset 16 is nonzero, clamps the word at a0 offset 8 to zero if below 1, otherwise calls a helper and sets it to 70 if at least 32; then clears bytes at offsets 19 and 20, sets the word at offset 10 to 632, loads pointers at offsets 96 and 100 from a table indexed by the word at offset 12, and zeroes offsets 30, 106 and 20.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `add_d0_to_indexed_words_loop`: Calls a library routine, then loops reading signed word offsets from a1, adding the returned value into longwords at a0 plus each offset, decrementing d1 until it reaches minus one.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_state_2f_and_decrement_x`: If the word at offset 104 of a0 equals 47, clears the word at offset 106 when the word at a0 is below minus 124, otherwise subtracts 4 from the word at a0.
- `check_state_25_and_clear_6a`: If the word at offset 104 of a0 equals 37 and the word at a0 is at least 124, clears the word at offset 106 of a0 and returns.
- `add_four_to_a0_word`: Adds four to the 16-bit word pointed to by a0 and returns to the caller.
- `decrement_timer_and_advance_phase`: Decrements the word at offset 58 of a0; when it goes negative it reloads from offset 56 and, if the word at offset 50 is not 78, increments that word, then returns.
- `countdown_then_select_animation_by_flags`: If the word at offset 60 of a0 is nonzero it decrements it and returns; otherwise it tests bits 7, 0 and 1 of byte 14, clears byte 19, sets a frame value and jumps to the animation setup routine.
- `decrement_timer_or_apply_input_animation`: If the word at offset 60 of the object is nonzero it decrements it and returns; otherwise it reads the byte at offset 14, tests bits 7, 0 and 2, toggles the word at offset 12, clears the byte at offset 19 and jumps to another routine with a frame value.
- `clamp_negative_word_at_offset_8`: Tests the signed word at offset 8 of the object and, if it is negative, writes zero to that word before returning.
- `start_delay_from_offset_32`: When the byte at offset 31 is set, the byte at offset 19 is clear and the word at offset 60 is zero, it stores 150 minus the word at offset 50 into offset 60 and sets the byte at offset 19.
- `init_animation_from_random_frame`: Calls a random number routine, picks a frame from one of two tables based on the word at offset 12, stores the frame and its script pointers at offsets 10, 96, 100 and 84, and clears offsets 30, 106 and 20.
- `set_animation_frame_and_script`: Stores d0 as the frame at offset 10, indexes the frame table using the word at offset 12, sets script pointers at offsets 96, 100 and 84, and clears offsets 30, 106 and 20.
- `set_animation_on_linked_object`: Saves d1, sets the animation frame on the current object, then loads the object pointer from offset 88 and applies the saved frame to that linked object.
- `advance_script_stream_and_dispatch`: Decrements the word at offset 106 of a0; when negative, reads signed words from a1, calls the jump-table target for each negative value, then stores the next word, long and pointer into offsets 104, 106, 124 and 96.
- `add_pitch_to_position_and_clear_state`: Adds the word at offset 124 of a0 to the word at a0, clears the long at offset 84 and the word at offset 124, then pops four bytes from the stack and returns.
- `add_pitch_to_position_load_pointer`: Adds the word at offset 124 of a0 to the word at a0 and loads the long at offset 100 of a0 into a1 before returning.
- `store_word_and_set_timer_fields`: Reads a word from the a1 pointer into d0, stores it at offset 16 of a0, and if nonzero clears offset 62 and sets offset 64 to -1 or 100 depending on a6 offset 24.
- `add_three_words_to_object`: Reads three consecutive words from a1 and adds them into the words at offsets 0, 4 and 8 of a0, advancing a1 by six bytes.
- `subtract_offset_7e_from_field_4`: Loads the word at offset 126 of a0 into d0 and subtracts it from the word at offset 4 of a0, then returns.
- `copy_two_longs_to_object_offsets`: Reads two consecutive longwords from the address in a1, stores the first at offset 70 and the second at offset 80 of the structure pointed to by a0, then returns.
- `update_max_word_from_stream`: Reads a word from the address in a1, compares it with the word at offset 4 of the structure in a0, and if greater stores it there before returning.
- `toggle_low_bit_at_offset_12`: Exclusive-ORs the word at offset 12 of the structure pointed to by a0 with the value 1, flipping its lowest bit, then returns.
- `copy_two_longs_to_object_offsets_2`: Reads two consecutive longwords from the address in a1 and stores them at offsets 66 and 76 of the structure pointed to by a0, then returns.
- `advance_position_and_load_frame_data`: Adds the word at offset 124 of a0 to the word at a0, reads a word from a1 into offset 10, then branches to another routine to load frame data.
- `store_word_to_object_offset_74`: Reads one word from the address in a1 and stores it at offset 74 of the structure pointed to by a0, then returns.
- `init_sprite_state_from_stream`: Sets byte at offset 20 of the a0 structure to 255, clears byte at offset 18, then copies the longword pointed to by a1 into offset 24 and advances a1.
- `advance_frame_counter_if_idle`: Clears byte at offset 20 of a0, tests byte at offset 18, and if zero increments byte at offset 22; then clears offset 18 and returns.
- `clear_byte_at_offset_21`: Clears the byte at offset 21 of the structure pointed to by a0 and returns immediately to the caller.
- `copy_word_from_offset_pointer_to_a0`: Loads the long pointer stored at offset 88 from a0 into a3, then copies the word at that pointer into the word at a0 and returns.
- `set_byte_at_indexed_offset_to_ff`: Reads a word from a1, increments a1, then writes byte value 255 to the address formed by adding that word to a0, and returns.
- `clear_byte_at_indexed_offset_to_zero`: Reads a word from a1, increments a1, then writes byte value zero to the address formed by adding that word to a0, and returns.
- `apply_relative_offset_if_state_matches`: Loads the pointer at offset 88 of a0, compares its word at offset 16 with a0's word at offset 16, and if equal stores that pointer's word at offset 8 plus the word read from a1 into a0's offset 8.
- `skip_word_offset_table_entry`: Reads two words from the table at a1, compares the first against the word at offset 8 of a0, and if not greater extends the second word and adds it to a1 before returning.
- `store_a1_pointer_at_offset_100`: Writes the a1 pointer into the longword at offset 100 of the a0 structure and returns to the caller.
- `decrement_timer_by_frame_delta`: Sets word at offset 20 of a6 to 1, loads pointer from offset 88 of a0, subtracts its offset 54 word from a0's offset 50 word, clamps to zero and returns.
- `init_and_decrement_animation_timer`: Sets a6 offset 20 to 1, on first entry stores 150 minus a0 offset 50 into offset 60 and sets flag byte 19, then subtracts a0 offset 54 from offset 50, clamping to zero.
- `advance_animation_and_load_frame`: Adds a0 offset 124 word to a0's first word, optionally calls the timer routine, advances a1 by two, reads a frame index, computes frame pointers and clears several a0 fields.
- `set_audio_control_and_play_sample`: Writes the value 65280 to offset 74 of the a6 structure, saves a0 on the stack, loads d0 with 1 and d1 from offset 34 of a0, calls a sound routine, then restores a0 and returns.
- `play_sample_from_a1_pointer`: Reads a word from the address in a1 and increments a1, saves a0 on the stack, loads d1 from offset 34 of a0, calls a sound routine, restores a0 and returns.
- `store_word_to_object_offset_28`: Copies the word pointed to by a1 into offset 28 of the structure pointed to by a0, increments a1, and returns to the caller.
- `select_attack_by_height_gap`: Reads two object x positions from a0 and a1, adds 208 to each, takes the absolute difference, then picks a jump-table offset based on the y field at offset 10 of a1 and calls the chosen routine with d2 set from that field.
- `resolve_contact_between_objects`: Compares d2 against 64 and 56, then checks facing, team and y fields of a0 and a1 to decide whether two objects touch, adjusting a1's y field and setting up a collision response before calling the shared routine.
- `check_contact_and_set_result`: If the zero flag is clear, calls the contact resolution routine, then compares the y fields of a0 and a1 and returns 112 or 88 in d3 depending on which is lower.
- `set_d3_58_if_vertical_gap_negative`: Branches to return if zero flag set, else calls a routine, sets d3 to 88, subtracts a1 offset 8 from a0 offset 8 into d4, and sets d3 to 112 if the result is negative.
- `set_d3_80_after_call`: Branches to return if zero flag set, otherwise calls a routine and loads d3 with 128 before returning to the caller.
- `resolve_attack_collision_by_d2`: Compares d2 against 64, 56 and 72 to pick attack branches, checks a0 and a1 offsets for range and facing, updates a6 fields and a1 state, then calls a routine.
- `pick_anim_for_small_x_offset`: If the word at offset 16 of a0 is zero, takes the absolute value of the word at a0 and, when it is at most 100, sets d3 to 120 and returns.
- `choose_anim_by_facing_and_x_delta`: Checks a0 offset 16 and a0 x word, then compares a0 offset 12 with a1 offset 12 and a0 offset 8 minus a1 offset 8 to set d3 to 120, 144, 136 or 104.
- `pick_anim_from_x_delta_sign`: When the zero flag is clear, sets d3 to 144, then subtracts the word at a1 offset 8 from a0 offset 8 and, if negative, sets d3 to 136 before returning.
- `select_attack_height_offset`: Compares the vertical positions at offset 8 of two objects and returns in d3 either 144 or 136 depending on which is higher, then returns.
- `check_objects_overlap_horizontally`: Compares two objects' vertical distance and sprite bounding boxes from a table, returning minus one in d2 when they overlap or zero otherwise.
- `resolve_attack_hit_and_damage`: Checks attacker and target proximity and bounding boxes, applies damage to the target's health, sets hit flags and selects a reaction animation index.
- `check_entity_x_range_and_y_band`: Reads word at offset 8 of a0 and returns -1 in d1 when it lies between 17 and 69 and the absolute value of the word at a0 is below 169, otherwise returns 0.
- `swap_entity_and_play_sound_by_type`: If the word at offset 10 of a1 equals 80, swaps a0 and a1, sets d0 to 40 or 304 depending on the swapped entity's type, calls the sound routine, then restores a0.
- `play_four_sound_effects_in_sequence`: Loads four PC-relative sound descriptors into a0 one after another and calls the sound routine for each, ending with a branch to the same routine for the last descriptor.
- `fighter_ai_state_dispatch`: Reads the fighter state at offset 10 of a0 and branches to per-state logic, handling attack, hit and block reactions, then updates the direction bits at offsets 14 and 15 of a0.
- `set_horizontal_proximity_flags`: Subtracts the x position at offset 8 of a1 from that of a0 and sets bit 0 of d0 if a0 is far left or bit 1 if far right, leaving d0 otherwise unchanged.
- `set_vertical_proximity_flags`: Subtracts the y position at offset 8 of a1 from that of a0 and sets bit 0 of d0 when a0 is far above, then branches to the shared exit.
- `skip_if_d1_above_five_else_set_bit1`: Compares d1 with 5 and returns if greater; otherwise sets bit 1 of d0 and returns.
- `wrestler_contact_ai_state_machine`: Compares positions and state fields of two wrestler structures at a0 and a1, sets action bytes and d0 flag bits, and branches to shared movement code.
- `advance_random_seed_and_pick_action`: Advances the 32-bit seed at offset 26 of a6 with shifts and exclusive-ors, returns low word in d1, then chooses an action byte for the wrestler at a0.
- `compute_relative_direction_flags`: Compares a0 x and y positions against a0 offsets 40 and 42, sets bits in d0 for direction, and clears byte 37(a0) when no valid target.
- `compare_x_position_set_direction`: Subtracts a1 x from a0 x, halves the difference, returns 128 and clears byte 37(a0) if equal, else returns 8 or 4 by sign.
- `proximity_attack_direction_check`: Checks byte 44(a0) once, then compares combined x positions with a1 within 75 units and calls two routines to derive a direction bitmask in d0.
- `branch_to_ai_state_dispatch`: Unconditionally branches to the AI state dispatch routine, transferring control without touching any registers or memory.
- `init_ai_state_and_pick_action_bit`: Sets AI state byte at offset 37 to 4, clears offset 44, then branches into the AI dispatcher; later path reads random bits to set a bit in d0 based on offset 12.
- `countdown_timer_then_set_action_bit`: Tests and decrements the byte timer at offset 44, reloads it with 20 and calls a routine when zero, otherwise sets a bit in d0 from offset 12 plus two.
- `clear_flag_if_sound_id_a0_a8_b0`: Compares the word at offset 10 of a1 against 160, 168 and 176; if any match it branches away, otherwise it clears the byte at offset 37 of a0 and returns.
- `decrement_timer_and_return_counter`: Tests the byte at offset 44 of a0; if zero it branches away, otherwise it decrements that byte, branches if it became zero, then returns the byte at offset 45 of a0 in d0.
- `set_state_and_reload_timer_from_a6`: If the byte at offset 46 of a0 is not 1, it sets it to 1, reads the word at offset 26 of a6, masks it to 4 bits, adds 10, stores it at offset 44 of a0 and clears offset 45.
- `clear_object_byte_at_offset_37`: Writes zero to the byte at offset 37 from the address in a0, then returns to the caller.
- `compare_positions_and_set_state_byte`: Clears the byte at offset 46 of a0, subtracts the word at a1 from the word at a0 into d1, takes its absolute value, and sets state bytes at offsets 45 and 46 based on comparisons with 98 and 42.
- `set_state_and_random_digit_byte`: Sets the byte at offset 45 of a0 to one, calls a routine, masks the returned d1 to four bits, adds ten, stores it at offset 44, and returns zero in d0.
- `advance_animation_frame_counter`: Increments the byte frame counter at offset 44 of the structure pointed to by a0, returns the byte at offset 45 in d0 when the counter is below 40, and clears the byte at offset 37 when the counter equals 45.
- `compare_positions_and_cycle_animation`: Compares halved words at offsets 0 and 40 of a0, sets d0 bits 3, 2, 1 or 0 based on position differences, and when positions match increments and masks the byte at offset 44, clearing offset 37 on wrap.
- `set_bit2_and_compare_vertical_position`: Sets bit 2 of d0, loads the word at offset 8 of a0 into d1, compares it with the word at offset 42, sets bit 0 of d0 if d1 is less, and returns.
- `set_bit1_in_d0_return`: Sets bit 1 of the low byte of d0 and returns, leaving the rest of d0 unchanged.
- `increment_and_wrap_counter_at_2c`: If d3 is nonzero, increments the byte at offset 44 of a0, masks it to two bits, and clears the byte at offset 37 when it wraps to zero.
- `compute_ai_direction_flags_into_14`: Reads fields of the a0 structure to set movement direction and speed words, builds direction bits in d0, stores them at offset 14, and handles a countdown timer at offset 44.
- `ciaa_fire_button_irq_toggle`: Saves a0 and d0, acknowledges INTREQ bit 3, reads CIAA_PRA fire button, pulses CIAA control lines, calls a library routine, then toggles a byte in a table indexed by the low seven bits of d0 and returns from interrupt.
- `acknowledge_intreq_bit5_rte`: Writes the value 32 to the custom chip interrupt request register to clear that interrupt and returns from the exception.
- `acknowledge_intreq_bit13_rte`: Writes the value 8192 to the custom chip interrupt request register to clear that interrupt and returns from the exception.
- `vblank_audio_volume_and_input_poll`: Vertical blank interrupt handler that ramps the audio volume toward a target, writes both volume registers, polls joystick and fire inputs into edge-detect state bytes, and calls three subroutines before returning.
- `start_audio_dma_channel_playback`: Waits for the audio interrupt slot to be free, sets the channel data pointer and length, enables the channel DMA bit in DMACON, installs the audio interrupt vector and acknowledges the CIA-B timer.
- `audio_interrupt_copy_next_sample`: Audio interrupt handler that copies ten bytes from the sample pointer to the audio hardware location, advances the pointer, re-enables the channel DMA bit, clears the interrupt request and chains to the next handler.
- `ciab_vertb_ack_and_copy_pointer`: Acknowledges the CIAB vertical-blank interrupt by writing to INTREQ, then copies five bytes from the pointer at offset 258 of a6 to the pointer at offset 258 and clears the word at offset 252 before returning from the exception.
- `cold_boot_hardware_and_vector_init`: Disables interrupts, clears all DMA and interrupt enables, programs CIAA and CIAB ports, fills the CPU exception vector table with a start address, installs two interrupt vectors, clears two longwords and enables DMA and interrupts before returning.
- `level_setup_copper_and_memory_copy`: Copies a table to low memory, installs interrupt vectors, programs the copper, bitplane and display registers, resets CIAB, copies two large memory blocks, calls two setup routines and stores pointers and a byte in the a6 structure.
- `scale_palette_by_fade_step`: Reads the signed fade byte at offset 230 of a6, adjusts it by one toward zero, then scales each of 32 words from the source pointer at 38(a6) by that factor and writes the results to the destination pointer at 42(a6).
- `or_four_bitplane_blocks_into_buffers`: Loads four source and destination pointer pairs and calls the bitplane merge routine three times to combine four blocks into three destination buffers.
- `or_four_bitplane_rows_into_buffer`: For 20 rows, ORs four longwords and a word from four source planes spaced 21120 bytes apart and stores the merged data to the destination pointer a4.
- `set_blitter_modulo_and_control_registers`: Points a0 at the custom chip base, writes all-ones to the blitter first and last word masks, sets blit control and modulo registers, clears blit data pointer, loads a source pointer from a6 offset and calls a routine.
- `update_ring_and_opponent_sprites`: Decrements frame counters, advances two opponent animation indices, allocates bitplanes and sprite pointers, builds four sprite descriptors and calls the sprite draw routine for both players.
- `blit_sprite_strips_to_bitplanes`: Configures blitter masks, control and modulo registers from a6 fields, sets four bitplane pointers and blit size, then repeats the setup for successive strips and jumps to the blitter wait routine.
- `step_vertical_scroll_toward_target`: Reads two joystick axis words, maps each to a direction index, looks up a target scroll value in a table, and if different sets a signed step and countdown in the object fields, otherwise applies the pending step to the scroll word.
- `build_scroll_strip_and_swap_buffers`: Writes a tile index from a table plus four successive screen addresses spaced by 52 bytes into the copper list at address 138, then swaps the two pairs of longwords at offsets 82 and 90 in the object.
- `scramble_bits_of_d0_into_d0_d1`: Masks d0 with 771, derives d1 by shifting and clearing bit 7, XORs it into d0, then rotates and adds bits to produce scrambled bytes in d0 and d1.
- `fill_rotating_bit_pattern_table`: Clears 79 ten-byte entries at the table pointed to by a0, then rotates d1, d2 and d3 right through carry to build a bit pattern, storing it each iteration.
- `copy_ten_byte_entry_by_index`: Multiplies d0 by ten, indexes the table at the pc-relative address, and copies the ten-byte entry into the buffer at a0 and the area ten bytes past a0.
- `pack_three_bits_into_word`: Adds d1 to d2, shifts and tests bits to set flags 1, 2 and 0 in d2, merges d1 into d0, swaps d0 and returns the packed word in d0.
- `advance_diwstrt_pointer_table`: Tests the a6+74 flag, and if set loads the next longword from the pc-relative table indexed by a6+75 into DIWSTRT, advances that index by four and clears the flag when it reaches forty.
- `jump_vector_entry_10`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_11`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `disk_track_read_and_decode`: Sets up CIAB and custom chip registers for disk DMA, waits on CIAB status bits, reads track data through a0 and decodes sector bytes into d5-d7 and d0 while toggling CIAB control bits.
- `advance_sector_column_counter`: Increments d6, and when it reaches twelve decrements d7, then increments d5 and resets d6, calling the sync routine when d5 is even before branching back to the decode loop.
- `dbra_loop_then_pop_d0_test_negative`: Decrements d7 and branches back to an earlier address while it is non-negative, then pops a word from the stack into d0 and branches if that value is negative; otherwise sets and clears bits in a byte at offset 256 from a4 before returning.
- `clear_set_bit0_at_a4_offset_256`: Clears bit 0 of the byte at offset 256 from a4, executes two no-ops, sets bit 0 of the same byte, loads the immediate value 2404 into d0 and falls through into the following routine.
- `store_d0_bytes_and_wait_bit1`: Writes the low byte of d0 to offset 1536 from a3 and the high byte to offset 1792, then polls bit 1 of the byte at offset 3328 from a3 until it is set before returning.
- `wait_vblank_then_set_audio_control`: Stores a1 and two control words at offsets in a5, polls bit 1 of the word at offset 30 until set, saves it, writes 16384, then sets bytes in a3 and waits on bit 1 at offset 3328.
- `decode_planar_bitmap_to_chunky_2`: Checks for a sync word in a1, advances past header and row offset, then for 128 iterations reads two longwords, interleaves their even bits into a0 and accumulates a checksum in d1.
- `copy_tiny_routine_and_reboot`: Calls a routine at low memory plus a fixed address, copies 414 bytes from a pc-relative table to address a fixed address, masks interrupts, pushes the reset vector and jumps to a system address.
- `sort_and_render_four_sprites`: Sorts four sprite entries by their word field, then for each computes screen x and y from object data and calls the blitter setup routines to draw them.
- `compute_sprite_clip_and_blit`: Reads sprite frame dimensions from a2, clips them against the screen, stores position and size fields in the object at a0, sets custom chip registers and calls the blitter routine.
- `blit_sprite_with_optional_mask`: Loads sprite position and size fields from a0, sets custom chip pointers and modulo registers, calls the blitter, then optionally blits a second masked copy.
- `queue_four_sprite_entries_from_entity`: Reads width, height and index from the entity at a0, computes screen offsets and a sprite pointer, then fills four copper-list entries at a5 offsets 64-88 and calls the copper queue routine.
- `reset_copper_list_and_emit_entries`: Waits on the copper busy bit, clears the list counter at a5+100, sets sentinel pointers, then walks the pointer list at a6+90 writing each entry to a5+84 and calling the copper emit routine until a negative terminator.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_display_and_run_level_loop`: Initializes the copper list and display registers, copies the copper table, then runs the level loop that waits on flags, calls the setup and copper routines and advances the level index.
- `poll_inputs_and_update_joystick_state`: Reads joystick and fire button state, merges direction flags, updates previous-state byte, calls two input helpers and dispatches through a table based on the global input word.
- `write_ciaa_pra_and_wait_clear`: Writes the value 240 to CIAA port A register at offset 230 from a6, then spins until that same byte reads back as zero before returning.
- `compute_sprite_pointer_from_index`: Scales d0 by eight, adds a table entry at pc-relative offset to a0, stores the resulting pointer at offset 38 from a6 and branches away.
- `blit_five_band_tiles_to_bitplanes`: Computes a tile source offset from d1 and d0, then programs the blitter five times to copy 40-pixel rows from a2 to a1 with d4 stride, waiting on DMACONR bit 14 before each write.
- `fetch_next_script_byte_and_dispatch`: Checks the active flag at offset 236 of a6, decrements the delay counter, reads the next byte from the pointer at 232, stores it back, and jumps through a 32-entry offset table indexed by that byte.
- `blit_character_glyph_to_five_bitplanes`: Uses d2 and d1 to index a 40-byte glyph in the font table, then copies its 40 bytes into five interleaved bitplane buffers at a2 with 8000-byte spacing and increments the counter at 238 of a6.
- `blit_data_table_fragment`: The bytes decode as meaningless immediate operands (ori and clr on registers and memory), indicating a data table fragment rather than executable code.
- `setup_blitter_and_call_copper`: Waits for the blitter to finish, programs blitter control, modulo and pointer registers with a source buffer, then calls a copper routine.
- `store_byte_and_advance_pointer`: Reads one byte from the address in a0, stores it at offset 237 of a6, saves the incremented a0 at offset 232 and returns.
- `dispatch_byte_to_jump_table`: Reads a byte from the address in a0, advances a0, tests bit zero, and if clear stores a flag and the new pointer in the a6 structure before jumping through a word offset table to a handler with preset d0-d4 values.
- `clear_byte_flag_at_a6_offset`: Clears the byte at offset 245 from a6 to zero and returns immediately to the caller.
- `set_byte_flag_at_a6_offset`: Sets the byte at offset 245 from a6 to all ones and returns immediately to the caller.
- `copy_a6_offset_f6_to_e8`: Copies the longword stored at offset 246 from register a6 into offset 232 of the same a6 base, then returns to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_12`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_forward_to_offset_ec`: Unconditionally branches forward by 236 bytes to another location in the same code segment; it does not return here.
- `branch_forward_to_offset_e2`: Unconditionally branches forward by 226 bytes to another location in the same code segment; it does not return here.
- `init_audio_channels_and_ciab`: Stores a0 globally, scans 128 bytes for the highest value, builds 31 channel pointers into a table, then configures CIAB, interrupt and DMA registers and silences all four audio volumes.
- `reset_audio_channel_pointers`: Loads the four audio channel register bases and their parameter blocks, calls the channel setup routine for each, then computes a sample offset from the current index and clears the audio period word.
- `copy_sample_pointer_and_check_type`: Loads a pointer from the table indexed by d1 into a6, reads its type byte, and if it equals fifteen stores the following byte as the current sound and clears the sound counter.
- `decode_and_apply_entity_record`: Reads a packed entity word from the input stream, decodes type and subtype, copies table fields into the a6 record, updates a5 output fields and DMACON, then dispatches to subroutines by subtype.
- `vertical_blank_dma_restore_interrupt`: Saves d0, installs the next interrupt vector, acknowledges CIAB, re-enables DMA via DMACON with bit 15 set, restores d0 and returns from interrupt.
- `ciab_interrupt_ack_and_chain`: Saves registers, reads the CIAB interrupt control register, calls the CIAB service routine, restores registers, acknowledges INTREQ bit and returns from interrupt.
- `update_audio_period_and_volume_state`: Loads four audio channel location and length registers from a table, adjusts a period counter, handles pending volume changes, wraps a channel index and compares a byte against a table entry before returning.
- `dispatch_sound_command_by_low_nibble`: Calls a helper, reads the command word at offset 2 of a6, masks it to twelve bits and branches to separate handlers based on the low nibble value, copying a6 offset 16 to a5 offset 6 for the default case.
- `read_joystick_direction_and_scale`: Reads the joystick port at a fixed hardware address, masks the low nibble of the input byte, and stores the resulting direction value into the object field at offset 6 of a5.
- `select_vertical_target_from_table`: Reads the masked word at offset zero of the object, scans a per-type table of 74-byte entries for the first threshold not above it, adjusts for a flag bit, stores the chosen word at offset 24 and sets the direction byte at offset 22.
- `step_vertical_position_toward_target`: If the target word at offset 24 is nonzero, adds or subtracts the speed byte at offset 23 from the position at offset 16, clamps to the target, clears the target, then optionally converts the position through a 72-byte table and stores it at offset 6 of a5.
- `apply_rotation_and_speed_controls`: Uses the angle byte at offset 27 and a sine table to compute a velocity from the speed nibble at offset 26, adds it to the position at offset 16, advances the angle, and handles further control bytes that update offsets 28, 29, 19, 32 and the global control bytes.
- `dispatch_joystick_command_nibbles`: Reads command nibbles from offsets 2 and 3 of the a6 structure, branches to per-command handlers, and for some commands toggles CIAA_PRA fire bit or updates bytes at offsets 30 and 31.
- `handle_audio_and_dma_commands`: Dispatches low nibble of byte 3(a6) to handlers that store values in a6 fields, check status bytes, and program DMACON at a fixed address with bit 15 set to enable DMA channels.
- `advance_and_flip_pointer_byte`: Uses high nibble of byte 31(a6) to index a table, adds the value to byte 35(a6), and when bit 7 sets, wraps pointer 36(a6) within the buffer and inverts the byte it points to.
- `bchg_mem_then_btst_offset`: Toggles bit d1 of the byte at a0, increments a0, then tests bit d1 of the longword at offset 762 from the new a0, setting the zero flag.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
