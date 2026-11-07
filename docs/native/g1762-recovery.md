# Game 1762 (bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01762). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (906800 top-level bytes in 73 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 40 of 40 named (0 author symbols), 40 of 40 described (5 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1043 of 1043 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 72864 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1583 of 1583 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 7402 |
| code | 5288 |
| data | 831867 |
| padding | 9507 |
| unused | 52736 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:PaRaDoX | 2 | 267 | 8 |
| Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:Bubba | 3 | 223 | 8 |
| Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 283 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 800 | 12 |
| Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:mblk1am.chr#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:mblk6am.chr#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:fgplax1a.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2287 -> 19598 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:set1.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6868 -> 15916 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:mfmap1.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6393 -> 40590 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:mparasp1.mac: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1342 -> 3392 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:mplxobj1.hs: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (797 -> 2704 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:mblk6am.pts: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4092 -> 6040 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:mblk1am.chr: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11914 -> 33408 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:complete.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (9876 -> 12544 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:level6.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (29974 -> 42930 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:sprites1.ant: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2714 -> 5214 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:mfmap6.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5463 -> 40960 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:mblk1am.pts: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3357 -> 4560 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:fgplax1b.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (400 -> 2462 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:sprites1.adt: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (49404 -> 96384 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:BubbaData/titledlt.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (21238 -> 57702 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:BubbaData/continue.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (22753 -> 30412 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:BubbaData/titlepic.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (7345 -> 11818 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:BubbaData/title.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (33588 -> 43062 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:BubbaData/getready.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2952 -> 3172 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:BubbaData/map.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (22744 -> 33760 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:BubbaData/gameover.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2911 -> 3188 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:mtrg1.trg: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (866 -> 1108 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:lev1info.obj: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (895 -> 2408 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:sprites6.ant: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2721 -> 4934 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:disk1.spk: high-entropy data file (7.85 bits/byte), classified as packed data
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:endseq.bit: high-entropy data file (7.66 bits/byte), classified as packed data
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:sprites6.adt: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (40190 -> 89064 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:mtrg6.trg: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1304 -> 1826 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:mparasp6.mac: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1570 -> 2048 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:set5.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (13987 -> 27660 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:lev6info.obj: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (937 -> 4650 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:mblk6am.chr: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (14871 -> 39456 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:level1.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (23559 -> 30916 bytes)
- runtime segment an address: 224 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 380 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:mblk1am.chr#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:mblk6am.chr#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf: blocks 11-11 (bytes 5632-6144) outside the filesystem hold data, not code (opcode density 0.020, entropy 0.84)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)[a]/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:Bubba: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 never-loaded region(s), 72864 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 4608 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:PaRaDoX

- `parse_amiga_hunk_relocations`: Walks the hunk list of a loaded executable, allocating memory for each code and data hunk through the system library and copying its contents.
- `busy_wait_countdown_loop`: Decrements a 16-bit register in a tight loop until its count wraps past zero, producing a pure busy-wait delay.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `unpack_backward_lz_stream`: Decodes a compressed byte stream into a buffer working backwards from its end.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `shift_bitstream_with_carry`: Shifts a multi-word bit buffer by a count taken from the low bits of a register, refilling from memory when the buffer empties.
- `stub_bad_instruction_data`: The decompiler reports bad instruction data around a trap call with no usable body.

### Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:Bubba

- `unpack_first_version_block`: Searches a linked list of blocks for one carrying a first-version four-byte signature.
- `extract_flag_bit_from_table`: Scans a table of word pairs until a masked value matches its partner, then reads a byte from the matching entry and uses it to extract a bit from a mask.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 255 into d0, then loads 16 into d1.
- `mask_and_conditional_call`: Ands two 16-bit values, and when one signed byte is less than another it calls a helper and returns that helper's word instead.
- `return_empty_routine`: It performs no operations that modify memory or registers and immediately returns.
- `build_bit_length_decode_tables`: Reads a count and then, for each of sixteen bit lengths, computes a bit mask and a shifted code value from a stack buffer.
- `apply_hunk_relocation_entries`: Walks a relocation list whose blocks carry a hunk type code.
- `stub_empty_return`: The routine body is only a return with no work, so it does nothing and is a stub.

### Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_copper_and_restore_dma`: Waits for the blitter to become idle, then saves the DMA control state and disables DMA and interrupts.
- `wait_status_then_set_bit`: Polls a status byte until it equals a fixed value and a second flag byte is nonzero.
- `wait_status_flags_ready`: Polls the same two status bytes as its neighbour and returns once the flag conditions hold.
- `run_until_flag_cleared`: Sets a flag word to one and repeatedly calls a helper routine, touching a custom chip register around each call, until the helper clears the flag.
- `initialize_bitplane_pointer_table`: Fills a fifteen-entry table with values that rise by a fixed stride, then writes a constant and a count into each of seven following eight-byte records.
- `render_text_stream_glyphs`: Waits on a delay counter, then reads characters from a stored string and treats a line-feed or zero byte as an end marker.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `select_alternating_plane_pointer`: Counts down a timer and, when it expires, toggles a byte and picks one of two buffer addresses to store into the copper pointer words.
- `pack_four_byte_record_flags`: Writes a four-byte record to the buffer at the address register: the low bytes of two inputs and of their sum, plus a flag byte recording the ninth bit of each value.
- `advance_state_machine_counter`: Steps a small state machine in which a counter moves a value up or down in steps of four depending on the current state.
- `setup_blit_control_block`: Waits for the blitter to become free, then stores a long pointer, a zero count and a constant into a control block and stores a combined word of two inputs into a field.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_pointer_table_by_threshold`: Searches a byte table for the first entry above a threshold, then clears a table of thirty pointer entries and stores the matching pointers into a second table.
- `restart_audio_channel_pointers`: Advances a frame counter and, when it runs out, waits in fixed delay loops around a DMA bit change.
- `process_audio_channel_effect`: Decodes one effect command from a stored word and applies it to a channel value.
- `sequence_audio_channel_step`: Reads the next packed sequence byte and, when its instrument nibble is nonzero, loads that instrument's pointer, length, volume and period values into the channel record.
- `apply_channel_effect_to_record`: Applies the effect nibble of a channel record.
- `copy_channel_word_field`: Copies one 16-bit field from the channel record at offset sixteen into the output structure at offset six and returns.
- `find_target_period_from_table`: Scans a row of a word table, selected by an index byte, for the first entry greater than the current value.
- `slide_value_toward_target`: Moves a stored value toward a target by a fixed step in one of two directions, clamping it when it arrives.
- `compute_oscillating_offset_value`: Computes a periodic offset from a waveform selected by two bits: a table lookup, a ramp, or a constant.
- `dispatch_effect_command`: Dispatches on the low nibble of a command byte.
- `copy_low_nibble_between_fields`: Reads a byte from a fixed offset in a record, keeps only its low four bits, and stores that value into another byte field of the same record.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
