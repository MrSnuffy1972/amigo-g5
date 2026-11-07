# Game 307 (World Soccer): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00307). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902800 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 17 raw-word lines |
| Functions named and described | pass, 18 of 18 named (0 author symbols), 18 of 18 described (2 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 291 of 291 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 17408 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 363 of 363 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 32331 |
| code | 2148 |
| data | 807905 |
| unused | 60416 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| World Soccer.adf:bootblock | 1 | 22 | 1 |
| logged disk load from World Soccer.adf boot-block load | 1 | 11 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 133 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 190 | 9 |
| World Soccer.adf bytes 436224-449024 (code never loaded by a reference run) | 1 | 0 | 0 |
| World Soccer.adf bytes 451072-455680 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- World Soccer/World Soccer.adf: custom-format (non-DOS) disk, 1588 non-empty blocks: matched against track-loaded segments
- World Soccer/World Soccer.adf: the boot block reads 32768 bytes from disk offset 814080 to an address (found statically); recovered as a unit
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 103 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 181 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- World Soccer/World Soccer.adf: bytes 436224-449024 look like code no run loaded: recovered position-independently (load address unknown)
- World Soccer/World Soccer.adf: bytes 451072-455680 look like code no run loaded: recovered position-independently (load address unknown)
- World Soccer/World Soccer.adf: 3 logged loads cover 50176 bytes of the image
- World Soccer/World Soccer.adf: 1 track-loaded blocks found in recovered runtime segments and classified through them
- World Soccer/World Soccer.adf: blocks 2-225 (bytes 1024-115712) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.08)
- World Soccer/World Soccer.adf: blocks 228-241 (bytes 116736-123904) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.90)
- World Soccer/World Soccer.adf: blocks 244-450 (bytes 124928-230912) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.93)
- World Soccer/World Soccer.adf: blocks 453-466 (bytes 231936-239104) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.90)
- World Soccer/World Soccer.adf: blocks 469-481 (bytes 240128-246784) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.84)
- World Soccer/World Soccer.adf: blocks 485-497 (bytes 248320-254976) outside the filesystem hold data, not code (opcode density 0.005, entropy 2.14)
- World Soccer/World Soccer.adf: blocks 500-513 (bytes 256000-263168) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.96)
- World Soccer/World Soccer.adf: blocks 516-528 (bytes 264192-270848) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.37)
- World Soccer/World Soccer.adf: blocks 532-846 (bytes 272384-433664) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.93)
- World Soccer/World Soccer.adf: blocks 849-849 (bytes 434688-435200) outside the filesystem hold data, not code (opcode density 0.008, entropy 0.39)
- World Soccer/World Soccer.adf: blocks 890-905 (bytes 455680-463872) outside the filesystem hold data, not code (opcode density 0.030, entropy 6.71)
- World Soccer/World Soccer.adf: blocks 907-920 (bytes 464384-471552) outside the filesystem hold data, not code (opcode density 0.054, entropy 5.60)
- World Soccer/World Soccer.adf: blocks 926-927 (bytes 474112-475136) outside the filesystem hold data, not code (opcode density 0.023, entropy 2.31)
- World Soccer/World Soccer.adf: blocks 962-962 (bytes 492544-493056) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.02)
- 2 never-loaded region(s), 17408 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 5 packed track region(s), 348160 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### World Soccer.adf:bootblock

- `allocate_and_run_boot_io`: Allocates a memory block through the exec library and, if that succeeds, fills in an I/O request and issues it with DoIO until the request's status byte clears, then calls the routine stored at the allocated address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_counter_low_byte`: Increments a 16-bit counter stored in the working block and keeps only its low byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `poll_fire_button_edges`: Alternates between waiting for bit six of a stored input value to go high and waiting for it to go low, calling an unnamed routine during each wait.
- `wait_for_position_field`: Busy-waits until a nine-bit field, taken from bits eight and up of a longword in the working block, equals 250, then returns.
- `read_mouse_axis_movement`: Reads the mouse port movement counter, turns the change since the previous reading into a signed step, and smooths that step into a stored position.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `find_matching_table_index`: Sets a stored index to 255, then scans three word tables backwards in steps of two from a fixed starting point until an entry passes the comparison test.
- `evaluate_flag_state`: Combines two flag bytes with AND and shifts the result to place a specific flag in its sign position.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 230 into d5, then merges 40 into d5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `acknowledge_pending_interrupts`: Combines the enabled interrupt mask with the interrupt request bits, writes the pending bits back to the interrupt request register to clear them, and handles them.
- `disable_all_interrupts_and_dma`: Writes a value with only the set-or-clear flag and every other bit set to the interrupt enable and DMA control registers, which clears every interrupt enable and every DMA channel.
- `advance_frame_counter_and_offset`: Advances a frame counter kept modulo 64 in the working block, then multiplies a bit field of a global by a row stride of 40 and adds a shifted value to get a new offset.
- `configure_blitter_strides_and_masks`: Writes a stored value to the blitter control register, sets the first-word mask to all ones and the last-word mask to zero, then gives the source and destination modulo registers the same 34-byte row stride.
- `blit_five_frame_indexed_stripes`: Programs five blitter operations one after another, all with the same control value and size.
- `reset_blitter_masks_dest_stride`: Clears the blitter control register, sets both edge masks to all ones, zeroes the source A modulo and sets the destination modulo to 34 bytes.
- `copy_stripes_from_fixed_source`: Starts five blits that all read from one fixed source address and write to destinations that advance by 8000 bytes each time, using a copy control value and a fixed blit size.
- `reset_blitter_masks_source_stride`: Clears the blitter control register, sets both edge masks to all ones, sets the source A modulo to 34 bytes and clears the destination modulo.
- `copy_stripes_to_fixed_target`: Starts five blits that read from a source advancing by 8000 bytes each time and write to one fixed destination address, with the same copy control value and blit size as its sibling blit routine.
