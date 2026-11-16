# Game 1341 (dragon breed (1989)(activision)[cr angels]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01341). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (907216 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 26 of 26 named (0 author symbols), 26 of 26 described (1 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 776 of 776 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 924 of 924 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 10804 |
| code | 3698 |
| data | 685866 |
| unused | 206848 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Dragon Breed (1989)(Activision)[cr Angels].adf:bootblock | 1 | 63 | 2 |
| logged disk load from Dragon Breed (1989)(Activision)[cr Angels].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 89 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 771 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: custom-format (non-DOS) disk, 1354 non-empty blocks: matched against track-loaded segments
- reference run failed (Karl 4 Nov 2026): run ext slid through memory (an address: 12 64 KB pages each with >= 29491 distinct PCs executed, 180961 of its 181749 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run ref ends at a 68000 address error (odd PC an address after an address): the harness halted there, nothing after it ran (68000 HALTED: address error at odd PC an address after an address, vector 3 an address is not a handler in RAM (the 68000 double-faults and halts); run ended at frame 12)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: the boot block reads 9216 bytes from disk offset 737280 to an address (found statically); recovered as a unit
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 89 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 629 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 branch displacement
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: 1 logged loads cover 9216 bytes of the image
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 11-80 (bytes 5632-41472) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.48)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 83-83 (bytes 42496-43008) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.47)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 85-88 (bytes 43520-45568) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.51)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 91-145 (bytes 46592-74752) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.98)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 150-154 (bytes 76800-79360) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.42)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 158-202 (bytes 80896-103936) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.54)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 207-211 (bytes 105984-108544) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.62)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 214-261 (bytes 109568-134144) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.63)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 263-307 (bytes 134656-157696) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.09)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 315-364 (bytes 161280-186880) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.84)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 367-370 (bytes 187904-189952) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.08)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 376-445 (bytes 192512-228352) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.69)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 451-496 (bytes 230912-254464) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.09)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 502-588 (bytes 257024-301568) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.05)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 594-696 (bytes 304128-356864) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.32)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 698-711 (bytes 357376-364544) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.75)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 713-727 (bytes 365056-372736) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.68)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 729-738 (bytes 373248-378368) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.51)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 740-743 (bytes 378880-380928) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.24)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 745-848 (bytes 381440-434688) outside the filesystem hold data, not code (opcode density 0.041, entropy 5.71)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 850-862 (bytes 435200-441856) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.22)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 876-876 (bytes 448512-449024) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.42)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 881-894 (bytes 451072-458240) outside the filesystem hold data, not code (opcode density 0.027, entropy 4.95)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 897-912 (bytes 459264-467456) outside the filesystem hold data, not code (opcode density 0.011, entropy 6.13)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 1037-1396 (bytes 530944-715264) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.69)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 1474-1474 (bytes 754688-755200) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 1496-1496 (bytes 765952-766464) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 1518-1518 (bytes 777216-777728) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 1540-1540 (bytes 788480-788992) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 1562-1562 (bytes 799744-800256) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 1584-1584 (bytes 811008-811520) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 1606-1606 (bytes 822272-822784) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 1628-1628 (bytes 833536-834048) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 1650-1650 (bytes 844800-845312) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 1672-1672 (bytes 856064-856576) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 1694-1694 (bytes 867328-867840) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 1716-1716 (bytes 878592-879104) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- dragon breed (1989)(activision)[cr angels]/Dragon Breed (1989)(Activision)[cr Angels].adf: blocks 1738-1738 (bytes 889856-890368) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- 2 packed track region(s), 68608 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Dragon Breed (1989)(Activision)[cr Angels].adf:bootblock

- `bootblock_load_and_launch_payload`: Opens dos.library, loads the boot payload to memory, byte-swaps it, enables DMA, runs it, then programs the copper and copies the next stage to high memory before jumping to it.
- `bootblock_second_stage_loader`: Uses dos.library to load two chunks into memory, byte-swaps the loaded block, then jumps through the entry pointer at offset twelve of the loaded image.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decompress_rle_bitstream_to_buffer`: Decodes a bit-packed run-length stream from the data table into a destination buffer, expanding literal and repeated bytes until the buffer is filled.
- `pop_word_ror_and_store_high_byte`: Pops a longword from the stack pointed to by a0, stores its high byte to the word at a5, rotates the longword right through the extend bit and returns the result in d0.
- `decode_run_length_encoded_stream`: Reads bytes from a1, expands run-length encoded data into memory at a0 until reaching a2, then jumps to the next stage; also handles bit-shift decoding via d0 and d1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `find_resident_and_jump_to_entry`: Saves all registers, calls a subroutine, opens dos.library via exec.FindResident using a6, loads the resident structure's entry pointer and jumps to it with d0 cleared.
- `open_graphics_library_and_set_copper`: Disables DMA and interrupts, opens graphics.library via exec, stores the base pointer, writes its copper list pointer to COP1LCH and restores DMA and interrupts.
- `main_loop_open_library_and_run_frame`: Opens graphics.library, installs a copper list, sets bitplane pointers, polls the fire button and VHPOSR, then calls the frame update routines until the button is pressed.
- `animate_sine_table_offset_by_phase`: Steps a phase counter through four states, adding or subtracting half the counter value to a running offset word to produce a triangle-wave animation value.
- `compute_sprite_offset_from_index`: Reads the word at the global index variable, subtracts sixteen repeatedly while it exceeds fifteen, then stores a derived offset and the index times sixteen into two globals.
- `build_sprite_descriptor_table`: Fills a table with entries of sprite number, byte, next number and zero, then appends a four-byte pointer looked up from a byte-indexed table, looping until the counter reaches the limit.
- `add_byte_offsets_to_every_eighth_byte`: Adds each byte from the source table to every eighth byte of the destination table, repeating 132 times, then returns.
- `shift_word_table_left_by_one_entry`: Copies each word of a table to the previous word position, then copies the first word to the end slot and returns.
- `advance_scroll_text_and_draw_tiles`: Decrements a scroll delay counter, reads the next byte from the text table, handles control codes to set scroll speed and palette, then calls drawing routines and advances the scroll offset and row counter.
- `map_text_char_to_tile_address`: Reads the next byte from the text table using the running index, then computes a tilemap address in a0 from the character code, handling space and letter ranges.
- `scale_byte_offset_to_screen_y`: Reads the word at memory location 28630, subtracts 16 repeatedly while it exceeds 15, then multiplies the remainder by 4096 and adds 3580, returning the result in d0.
- `advance_sprite_pointer_by_stride`: Reads a word from the address in a2, multiplies it by 48 and adds it to a1, then adds the word at 286e6 to a2 and returns.
- `blitter_copy_three_bitplanes`: Programs the blitter registers with the value in d0, zero modulo and first/last masks, sets channel pointers from a1 and a0, and starts a blit of size 898.
- `setup_blitter_dual_playfield_a`: Writes blitter control, modulo, pointer and size registers to configure a dual playfield blit using source pointers at a fixed address and a fixed address, then returns.
- `setup_blitter_for_full_screen_copy`: Writes blitter control, modulo, pointer and size registers to configure a full-screen blit with both channel masks enabled and zero modulo.
- `branch_to_shared_blitter_setup`: Executes a single unconditional branch to a shared routine at a later address, transferring control without touching any registers or memory.
- `silence_audio_and_clear_dma`: Clears a status word, zeroes the four audio period registers, sets DMACON to enable only the low four DMA bits, clears the CIAA fire-button bit and returns.
- `init_four_voice_audio_engine`: Copies sample tables, builds four voice structures with pointers, periods and volumes, programs the audio hardware registers, enables DMA and interrupts, then runs a per-frame voice update that reloads pointers and periods.
- `advance_voice_sample_pointer`: Steps one voice's sample pointer by its offset, wraps the position and sequence counter, decodes the next sample header, updates volume, period and pointer fields, and enables the voice's DMA bit.
- `parse_sound_sequence_and_update_channel`: Walks the sound sequence bytes at a0+18 using the offset at a0+50, handling command bytes to load samples, set DMACON bits, jump sequences, set delays and compute the period written to a0+56, returning the volume byte in d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `indirect_jump_stub`: Jump stub: passes control to an address held in or computed from register a4, so the target is only known at run time.
