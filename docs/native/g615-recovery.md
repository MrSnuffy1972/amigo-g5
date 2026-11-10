# Game 615 (Super Hang-On): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00615). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 1 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 52 of 52 named (0 author symbols), 52 of 52 described (7 by rule, 45 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2 of 2 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 2048 bytes, 10 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1214 of 1235 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1500 |
| code | 4132 |
| data | 889340 |
| padding | 4 |
| unused | 6144 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Super Hang-On.adf:bootblock | 1 | 77 | 3 |
| logged disk load from Super Hang-On.adf boot-block load | 1 | 305 | 25 |
| logged disk load from Super Hang-On.adf boot-block load | 1 | 500 | 14 |
| Super Hang-On.adf bytes 11264-11776 (code never loaded by a reference run) | 1 | 0 | 0 |
| Super Hang-On.adf blocks 23-25 (bytes 11776-13312) (code never loaded by a reference run, load address unknown) | 1 | 353 | 10 |

## Cases handled

- Super Hang-On/Super Hang-On.adf: custom-format (non-DOS) disk, 1746 non-empty blocks: matched against track-loaded segments
- Super Hang-On/Super Hang-On.adf: the boot block reads 4608 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- Super Hang-On/Super Hang-On.adf: the boot block reads 2048 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- Super Hang-On/Super Hang-On.adf: bytes 11264-11776 look like code no run loaded: recovered position-independently (load address unknown)
- Super Hang-On/Super Hang-On.adf: 3 logged loads cover 7168 bytes of the image
- Super Hang-On/Super Hang-On.adf: blocks 15-21 (bytes 7680-11264) hold ASCII text
- Super Hang-On.adf blocks 23-25 (bytes 11776-13312) (code never loaded by a reference run, load address unknown): never-loaded code-like bytes recovered position-independently (load address unknown, 1026 of 1536 bytes decode)
- Super Hang-On/Super Hang-On.adf: blocks 33-341 (bytes 16896-175104) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.54)
- Super Hang-On/Super Hang-On.adf: blocks 343-569 (bytes 175616-291840) outside the filesystem hold data, not code (opcode density 0.010, entropy 5.27)
- Super Hang-On/Super Hang-On.adf: blocks 571-640 (bytes 292352-328192) outside the filesystem hold data, not code (opcode density 0.048, entropy 6.31)
- Super Hang-On/Super Hang-On.adf: blocks 644-1759 (bytes 329728-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.74)
- 1 never-loaded region(s), 512 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Super Hang-On.adf:bootblock

- `allocate_and_load_boot`: Stores a flag into a structure, calls two library allocation routines, and if the second allocation fails it records the returned address and jumps to a loader routine via a trap.
- `setup_and_jump_loader`: Disables interrupts and DMA, sets COLOR00, DMACON and the blitter origin register, clears the blitter completion flag, relocates the stack pointer, and jumps to the loader at a fixed address.
- `wait_vbl_and_call_library`: Waits for the vertical retrace in a loop, then stores the library base address and two pointers into a global structure, calls the library entry twice, and finally jumps to the routine at a fixed address.

### logged disk load from Super Hang-On.adf boot-block load

- `init_and_update_frame_delta`: Calls the exec library entry point, disables DMA, zeroes a block of memory, programs the blitter, and then updates a signed frame delta from the VBLANK counter while clamping it to zero.
- `store_routine_addresses`: Loads thirteen routine addresses into a table and stores each longword into consecutive slots, then returns.
- `zero_filled_data_area`: Contains only zero-filled words that decode as no-ops, so this range is most likely a zero-filled data area that other code refers to by address.
- `zero_filled_data_area_2`: Contains only zero-filled words that decode as no-ops, so this range is most likely a zero-filled data area that other code refers to by address.
- `clear_flag_and_store_bytes`: Clears a 16-bit field at offset 198 of the structure in a5, then stores the bytes 32, 78 and 79 at the address in a0.
- `clear_flag_and_store_bytes_2`: Clears a 16-bit field at offset 19c of the structure in a5, then stores the bytes 32, 78 and 79 at the address in a0.
- `clear_flag_and_store_bytes_3`: Clears a 16-bit field at offset 19a of the structure in a5, then stores the bytes 32, 78 and 79 at the address in a0.
- `set_flag_to_one`: Stores the value one into a 16-bit field at offset 19e of the structure in a5, then returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `set_flag_and_store_bytes`: Stores the value one into a 16-bit field at offset 198 of the structure in a5, then stores the bytes 89, 69 and 83 at the address in a0.
- `set_flag_and_store_bytes_2`: Stores the value one into a 16-bit field at offset 19c of the structure in a5, then stores the bytes 89, 69 and 83 at the address in a0.
- `write_string_and_flag`: Stores the word value one at offset 410 of the structure in a5, then writes the bytes 89, 69 and 83 to three consecutive RAM locations starting at a fixed address, and returns.
- `set_flag_word`: Stores the word value one at offset 414 of the structure in a5, and returns.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `dispatch_via_jumptable`: Decrements a counter in a structure and returns when it is not zero, otherwise reads two counters from the structure and jumps through an indirect jump table using their scaled difference.
- `update_frame_delta`: Reads the VBLANK frame counter and a stored byte, then updates a signed delta at offset 906 of the structure in a5, clamping it to zero when negative and raising it to a newer counter value if needed.
- `fill_and_adjust_slots`: Loops over ten word slots, filling each with the value 4095 and copying a source word into matching slots, then adds an offset value to a counter and negates it when it reaches 3584 or 3822, and returns.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 22 into d6, then merges 54 into -(a6), then merges 86 into d6.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `fill_blocks_from_table`: Loops nine times, reading a byte from a table and storing a value into a structure field indexed by a stored word, then overwrites that field with another value when a shifted difference is odd before calling a helper routine.
- `program_blitter_and_copy`: Programs the blitter registers with fixed control and pointer values, then increments a counter and, when its low bits are clear, loads a source address and calls a copy routine.
- `copy_block_data`: Loops over a number of blocks, copying 48 bytes from a source table into a destination area each time, scaling each block by subtracting 32 from a source index and advancing the pointers by fixed amounts per iteration.

### logged disk load from Super Hang-On.adf boot-block load

- `check_eight_byte_boot_signature`: Calls two unnamed routines, stores a value in a global, and returns it if negative.
- `find_entry_then_transfer_data`: Stores a destination pointer in a global, then scans a table of four-byte entries until it finds one matching a passed value or exhausts the entries, returning an error code when none match.
- `init_disk_controller_registers`: Clears an interrupt request, writes a fixed byte value to a CIA port B register, and stores the disk length, sync word and DMA control values into their registers, then returns.
- `write_disk_drive_control_values`: Clears an interrupt request, writes three fixed byte values to a CIA port B register, and stores a value into the disk length register, then returns.
- `copy_sectors_after_seek_2`: Compares a passed value against a stored one and returns it when they match.
- `process_disk_commands`: Compares a passed value against a stored one, then repeatedly scans a command table, computing a checksum and writing transformed words to a buffer until the checksum matches or a retry counter is exhausted.
- `validate_request_then_read_track`: Loads the value 249 into d0 and the stored position into d1, then tests bit 0 of d2.
- `seek_to_target_position`: If the short at a1 is negative it polls the CIA port A register until bit 4 is set, then clears that location.
- `wait_vblank_and_set_control`: Polls the CIA port A register until bit 4 is set, storing 117 or 119 into the CIA port B register depending on the result, then returns zero.
- `check_cia_port_bit`: Tests bit 2 of a CIA port register and returns when that bit is clear, otherwise falling through into the following routine.
- `write_drive_control_and_call`: Writes the byte 112 then 117 to the CIA port B register at a fixed address, calls one unnamed routine, and returns zero.
- `check_cia_port_bit_2`: Tests bit 2 of a CIA port register and returns when that bit is clear, otherwise falling through into the following routine.
- `write_alt_control_and_call`: Writes the byte 114 then 119 to the CIA port B register at a fixed address, calls one unnamed routine, and returns zero.
- `wait_for_interrupt_flag`: Polls a global variable in a loop until bit 5 is set, then clears that bit, and returns.

### Super Hang-On.adf blocks 23-25 (bytes 11776-13312) (code never loaded by a reference run, load address unknown)

- `read_sectors_in_chunks`: Computes a sector count from a passed value, repeatedly calls a read helper while it reports an error, then splits the remaining sectors into chunks and returns zero.
- `copy_sectors_after_seek`: If the requested value differs from a stored one it calls a helper routine and returns its negative error, otherwise it copies 512 bytes from a source region computed from the registers into the destination pointer and returns the requested value.
- `process_disk_sector_data`: Waits for the CIA port A status bit to clear, then repeatedly scans a table of 16-bit entries for a matching value and, when found, mixes its contents with a second table using XOR and interleaved bit operations into a third location, retrying on failure until a match succeeds or a counter is exhausted.
- `poll_cia_and_adjust_slot`: If the 16-bit value pointed to by a1 is negative, it repeatedly calls another routine while polling bit 4 of the CIA port A register until that bit is set, then clears the value at a1.
- `set_cia_portb_modes`: Writes 117 to the CIA port B register and waits for vertical blank, then writes 119 to the alternate CIA port B control address and waits for vertical blank, but only when bit 4 of a global variable is set; otherwise it writes 119 to the same alternate port and waits for vertical blank.
- `test_cia_port_a_bit`: Tests bit 2 of the CIA port A register and returns when that bit is clear.
- `write_cia_portb_and_wait_vblank`: Stores the byte 117 into the CIA port B register, calls a routine that waits for the vertical blank, and returns zero.
- `test_cia_port_bit2`: Tests bit 2 of the CIA port register at a fixed address and returns immediately when that bit is clear, otherwise it falls through into the following routine.
- `write_alt_cia_and_call_vblank`: Stores the byte 119 into the CIA port B register at the fixed control address, calls the routine that waits for the VBLANK bit and then sets the DMA bit, and returns zero.
- `wait_vblank_bit5`: Waits until bit 5 of the VBLANK status register is set, then sets bit 5 of the DMA control register and returns; if that bit is not set it checks bit 6 of the input port register and returns without changing anything.
