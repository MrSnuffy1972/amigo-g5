# Game 1680 (super hang on (1988)(activision)[cr defjam - ccs][m ela team - fbi soft]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01680). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905664 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 10 raw-word lines |
| Functions named and described | pass, 47 of 47 named (0 author symbols), 47 of 47 described (9 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 102 of 102 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 964 of 964 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1685 |
| code | 5006 |
| data | 892827 |
| padding | 2 |
| unused | 6144 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf:bootblock | 1 | 40 | 2 |
| logged disk load from Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf boot-block load | 1 | 129 | 6 |
| logged disk load from Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf boot-block load | 1 | 469 | 16 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 375 | 23 |

## Cases handled

- super hang on (1988)(activision)[cr defjam - ccs][m ela team - fbi soft]/Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf: custom-format (non-DOS) disk, 1746 non-empty blocks: matched against track-loaded segments
- 102 executed addresses below an address (an address) run the game's own code, not harness stubs (their opcode words are not the fake OS's): treated as game code (LOWCODE, 15 Nov 2026)
- super hang on (1988)(activision)[cr defjam - ccs][m ela team - fbi soft]/Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf: the boot block reads 1024 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- super hang on (1988)(activision)[cr defjam - ccs][m ela team - fbi soft]/Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf: the boot block reads 2048 bytes from disk offset 11264 to an address (found statically); recovered as a unit
- runtime segment an address: 102 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- super hang on (1988)(activision)[cr defjam - ccs][m ela team - fbi soft]/Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf: 2 logged loads cover 3072 bytes of the image
- super hang on (1988)(activision)[cr defjam - ccs][m ela team - fbi soft]/Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf: 8 track-loaded blocks found in recovered runtime segments and classified through them
- super hang on (1988)(activision)[cr defjam - ccs][m ela team - fbi soft]/Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf: blocks 4-21 (bytes 2048-11264) hold ASCII text
- super hang on (1988)(activision)[cr defjam - ccs][m ela team - fbi soft]/Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf: blocks 33-341 (bytes 16896-175104) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.54)
- super hang on (1988)(activision)[cr defjam - ccs][m ela team - fbi soft]/Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf: blocks 343-536 (bytes 175616-274944) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.25)
- super hang on (1988)(activision)[cr defjam - ccs][m ela team - fbi soft]/Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf: blocks 539-549 (bytes 275968-281600) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.41)
- super hang on (1988)(activision)[cr defjam - ccs][m ela team - fbi soft]/Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf: blocks 556-569 (bytes 284672-291840) outside the filesystem hold data, not code (opcode density 0.044, entropy 5.42)
- super hang on (1988)(activision)[cr defjam - ccs][m ela team - fbi soft]/Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf: blocks 571-640 (bytes 292352-328192) outside the filesystem hold data, not code (opcode density 0.048, entropy 6.31)
- super hang on (1988)(activision)[cr defjam - ccs][m ela team - fbi soft]/Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf: blocks 644-1759 (bytes 329728-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.74)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `init_disk_controller_registers` (Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf boot-block load +an address to an address (2048 bytes)): Writes a fixed sequence of values to the disk length, sync word, ADKCON, DMA control and interrupt request registers. Kind: disk hardware signature/register check. Pass path: the routine continues execution to the next instruction (e.g., bsr.w an address) if the hardware registers match the expected values.
- `write_disk_drive_control_values` (Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf boot-block load +an address to an address (2048 bytes)): Writes three fixed values to the CIA port B and disk length registers and the DMA control register. Kind: disk hardware/parameter check. Pass path: bra.w an address.
- `read_track_sync_scan` (Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf boot-block load +an address to an address (2048 bytes)): Sets the disk DMA pointer, length and sync registers, waits on a drive status bit, and scans the read buffer for the disk sync pattern. Kind: checksum/data-integrity trap. Pass path: the loop continues or returns in_D1 if sVar3 reaches -1 without the uVar1 check failing.
- `write_drive_control_and_call` (Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf boot-block load +an address to an address (2048 bytes)): Writes a fixed control value to the global used for the CIA port B drive register and calls one unnamed routine, then returns zero. Kind: disk hardware/track check. Pass path: bra.b an address.
- `write_alt_control_and_call` (Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf boot-block load +an address to an address (2048 bytes)): Writes the second fixed control value to the global used for the CIA port B drive register, calls one unnamed routine, and returns zero. Kind: loader/checksum trap. Pass path: sub_052A branch an address.

## Functions


### Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf:bootblock

- `boot_alloc_and_do_io`: Boot block entry that allocates memory through exec's AllocMem, performs an I/O request with DoIO, and calls one further library function.
- `configure_display_and_dma_registers`: Writes the bitplane control, background colour and DMA and interrupt enable registers as a short boot-block setup.

### logged disk load from Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf boot-block load

- `initialize_copper_list`: Initializes a copper list starting at a global memory location, setting up color registers and waits while managing a counter and pointer through multiple conditional branches.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (sub.w) and then falls through into the code that follows, with no return of its own.
- `append_copper_color_wait_mod`: Writes d0, the copper wait command, and d1 through a2, advances d0 by 256, and returns for the modified release.
- `append_color_register_pair_mod`: Invokes the modified wait writer, appends two colour-register pairs with d3, subtracts 273 from d3, adds 512 to d0, and returns.
- `call_builder_adjust_d2_a2_mod`: Calls the modified builder, increases d2 by 384, decreases a2 by d4, and falls through into the next routine body.
- `invoke_two_os_operations_mod`: Places a3, d2 and d3 for the first library call, a2 and d4 for the second, advances a2 by d4, and returns.

### logged disk load from Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf boot-block load

- `load_disk_boot_sectors`: Validates a boot signature and searches for specific data markers in memory to identify disk chunks.
- `check_eight_byte_boot_signature`: Calls unnamed routines, stores a value in a global, and returns it if negative, otherwise compares eight bytes of a global buffer in a loop and returns an error code on the first mismatch.
- `lookup_record_and_process_chunks`: Searches a fixed table of records for the key given in the input and returns an error code when none matches.
- `init_disk_controller_registers`: Writes a fixed sequence of values to the disk length, sync word, ADKCON, DMA control and interrupt request registers.
- `write_disk_drive_control_values`: Writes three fixed values to the CIA port B and disk length registers and the DMA control register.
- `copy_sectors_after_seek`: Copies a run of 512-byte blocks from a fixed source region into a destination pointer.
- `read_track_sync_scan`: Sets the disk DMA pointer, length and sync registers, waits on a drive status bit, and scans the read buffer for the disk sync pattern.
- `validate_request_then_read_track`: Validates the request value and stores a fallback value in a global when a flag bit is clear, returning an error if the request is zero.
- `seek_to_target_position`: Compares a requested position with a stored one and calls an unnamed routine once for each unit of difference, updating the stored position as it goes.
- `choose_drive_control_value`: Checks a status bit of the CIA port A register and writes one of two drive control values to a global depending on it.
- `set_control_if_status_bit`: Checks bit two of the CIA port A status register and, when it is set, writes a fixed control value to a global and calls an unnamed routine.
- `write_drive_control_and_call`: Writes a fixed control value to the global used for the CIA port B drive register and calls one unnamed routine, then returns zero.
- `set_alt_control_on_status`: Checks bit two of the CIA port A status register and, when it is set, writes a second fixed control value and calls an unnamed routine.
- `write_alt_control_and_call`: Writes the second fixed control value to the global used for the CIA port B drive register, calls one unnamed routine, and returns zero.
- `wait_for_interrupt_flag`: Polls the interrupt request register until one bit is set, then writes the same bit back to acknowledge it.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_tst_bne_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then returns.
- `frag_btst_bne_rte`: Short code fragment that tests bit 4 of a fixed memory address, then branches if the last result was not zero, then tests bit 5 of a fixed memory address, then branches if the last result was not zero, then tests bit 6 of a fixed memory address, then branches if the last result was not zero, then returns from the exception.
- `wait_for_interrupt_flag_2`: Polls a global variable in a loop until bit 5 is set, then clears that bit, and returns.
- `frag_clr_move_rts`: Short code fragment that clears a fixed memory address (2 times in a row), then copies 4178 to a fixed memory address, then copies 252 to a fixed memory address, then copies 13 to a fixed memory address, then copies 1 to a fixed memory address, then copies 130 to a fixed memory address, then copies 32776 to a fixed memory address, then returns.
- `frag_clr_move_rts_2`: Short code fragment that clears a fixed memory address, then copies 50332 to d0, then copies d0 to a fixed memory address (2 times in a row), then copies 10249 to a fixed memory address, then copies 9 to a fixed memory address, then copies 2 to a fixed memory address, then clears a fixed memory address (2 times in a row), then returns.
- `handle_input_interrupt`: Processes input interrupt flags from CIA hardware and updates a global counter, calling other routines based on the counter's value and conditions.
- `frag_addq_bne_move_rts`: Short code fragment that adds 1 to the memory at a0, then branches if the last result was not zero, then copies 156 to the memory at a0, then adds 1 to the memory just below a0 (moving a0 down), then branches if the last result was not zero, then copies 196 to the memory at a0, then adds 1 to the memory just below a0 (moving a0 down), then returns.
- `frag_clr_move_bra`: Short code fragment that clears a fixed memory address, then copies 65280 to a fixed memory address, then jumps to another point in the code.
- `calculate_scaled_offset_value`: Checks three global bytes for non-zero values to determine whether to call frag_move_rts with specific constants; otherwise, it subtracts 2048 from a value loaded from a table, calculates the absolute value divided by 7, and stores the result in a global word.
- `frag_tst_bne_move_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then copies 0 to the condition codes, then returns.
- `frag_moveq_mulu_move_subq_lsr`: Short code fragment that loads 7 into d2, then multiplies d2 by a location in the code (unsigned), then copies d2 to d3, then subtracts 1 from d2, then shifts d3 right by 1.
- `frag_move_rts`: Short code fragment that copies d2 to a fixed memory address, then copies d3 to a fixed memory address, then returns.
- `frag_moveq_move_clr_not_lea_lsr_scc_rts`: Short code fragment that loads 0 into d0, then copies a fixed memory address to d0, then clears a fixed memory address, then copies 64 to a fixed memory address, then inverts d0, then copies d0 to a fixed memory address, then loads the address of a location in the code into a0, then shifts d0 right by 1, then sets an indexed entry from a0 if the carry flag is clear, then returns.
- `read_joystick_input`: Processes joystick and mouse input from hardware registers and stores the resulting control state in a global variable.
- `initialize_graphics_registers`: Initializes various graphics hardware registers including bitplane control, display window settings, copper list pointers, and sprite configurations.
- `setup_copper_list`: Configures a copper list pointer by loading an address from program memory into a global variable, then writes a specific value to a hardware register to point the copper interpreter at that list.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `copy_graphics_data`: Copies a block of graphics data from program memory to a specific memory location in the custom chipset area.
- `frag_st_tst_bne_clr_move_rts`: Short code fragment that sets a fixed memory address to all ones, then tests a fixed memory address, then branches if the last result was not zero, then clears a fixed memory address, then copies 416 to a fixed memory address, then returns.
- `frag_st_move_rts`: Short code fragment that sets a fixed memory address to all ones, then copies d0 to a fixed memory address, then returns.
- `frag_tst_bne_rts_2`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then returns.
- `enter_debug_mode`: Checks a global variable and enters debug mode if the variable is zero, otherwise it returns immediately.
- `empty_routine`: Contains only a return instruction and does nothing else.
