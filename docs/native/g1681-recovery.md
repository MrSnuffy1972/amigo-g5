# Game 1681 (super hang on (1988)(activision)[cr defjam - ccs][t +2 band]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01681). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905664 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 13 raw-word lines |
| Functions named and described | pass, 48 of 48 named (0 author symbols), 48 of 48 described (14 by rule, 10 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 102 of 102 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 2048 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1036 of 1036 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2679 |
| code | 4352 |
| data | 892483 |
| padding | 6 |
| unused | 6144 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf:bootblock | 1 | 46 | 2 |
| logged disk load from Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf boot-block load | 1 | 469 | 16 |
| logged disk load from Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf boot-block load | 1 | 147 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 374 | 25 |
| Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf bytes 1024-2048 (code never loaded by a reference run) | 1 | 0 | 0 |
| Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf bytes 283136-284160 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- super hang on (1988)(activision)[cr defjam - ccs][t +2 band]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf: custom-format (non-DOS) disk, 1746 non-empty blocks: matched against track-loaded segments
- 102 executed addresses below an address (an address) run the game's own code, not harness stubs (their opcode words are not the fake OS's): treated as game code (LOWCODE, 15 Nov 2026)
- super hang on (1988)(activision)[cr defjam - ccs][t +2 band]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf: the boot block reads 2048 bytes from disk offset 11264 to an address (found statically); recovered as a unit
- super hang on (1988)(activision)[cr defjam - ccs][t +2 band]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf: the boot block reads 3584 bytes from disk offset 2048 to an address (found statically); recovered as a unit
- runtime segment an address: 102 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- super hang on (1988)(activision)[cr defjam - ccs][t +2 band]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf: bytes 1024-2048 look like code no run loaded: recovered position-independently (load address unknown)
- super hang on (1988)(activision)[cr defjam - ccs][t +2 band]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf: bytes 283136-284160 look like code no run loaded: recovered position-independently (load address unknown)
- super hang on (1988)(activision)[cr defjam - ccs][t +2 band]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf: 4 logged loads cover 7680 bytes of the image
- super hang on (1988)(activision)[cr defjam - ccs][t +2 band]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf: 6 track-loaded blocks found in recovered runtime segments and classified through them
- super hang on (1988)(activision)[cr defjam - ccs][t +2 band]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf: blocks 11-21 (bytes 5632-11264) hold ASCII text
- super hang on (1988)(activision)[cr defjam - ccs][t +2 band]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf: blocks 33-341 (bytes 16896-175104) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.54)
- super hang on (1988)(activision)[cr defjam - ccs][t +2 band]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf: blocks 343-536 (bytes 175616-274944) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.25)
- super hang on (1988)(activision)[cr defjam - ccs][t +2 band]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf: blocks 539-549 (bytes 275968-281600) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.41)
- super hang on (1988)(activision)[cr defjam - ccs][t +2 band]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf: blocks 556-569 (bytes 284672-291840) outside the filesystem hold data, not code (opcode density 0.044, entropy 5.42)
- super hang on (1988)(activision)[cr defjam - ccs][t +2 band]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf: blocks 571-640 (bytes 292352-328192) outside the filesystem hold data, not code (opcode density 0.048, entropy 6.31)
- super hang on (1988)(activision)[cr defjam - ccs][t +2 band]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf: blocks 644-1759 (bytes 329728-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.74)
- 2 never-loaded region(s), 2048 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `set_disk_read_configuration` (Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf boot-block load +an address to an address (2048 bytes)): Stores a sequence of disk-controller values, including a zero, a length value, a sync pattern and control words, into consecutive global cells. Kind: disk hardware configuration/parameter setup. Pass path: execution continues to the next instruction (e.g., bsr.w an address or st.b an address.l).
- `store_disk_length_config` (Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf boot-block load +an address to an address (2048 bytes)): Stores three fixed values into consecutive global cells, forming a disk length configuration, and returns. Kind: disk hardware/length configuration. Pass path: bra.w an address.
- `read_disk_track_sectors` (Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf boot-block load +an address to an address (2048 bytes)): Waits for the disk controller, issues a sync-word read, then searches for the sync pattern and decodes each sector's odd and even bit fields into the buffer. Kind: checksum/data integrity trap. Pass path: the branch at an address (bpl.b an address).
- `store_state_code_and_call` (Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf boot-block load +an address to an address (2048 bytes)): Stores a fixed state code in a global cell, calls another routine, and returns zero. Kind: disk hardware/sync check. Pass path: bra.b an address.
- `write_port_status_byte` (Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf boot-block load +an address to an address (2048 bytes)): Stores a constant byte into a global variable and then calls another routine. Kind: disk hardware/track check. Pass path: the branch following the call that continues normal execution (e.g., the instruction immediately after the BSR/CALL).

## Functions


### Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf:bootblock

- `allocate_memory_and_run_io`: The decompiler timed out on this 26-instruction boot-block entry.
- `configure_display_and_dma`: The decompiler timed out on this 20-instruction routine.

### logged disk load from Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf boot-block load

- `initialize_boot_sequence`: Verifies the boot name, searches for specific signature strings in memory to locate entry points, and performs a block memory copy.
- `check_eight_byte_boot_signature`: Calls two unnamed routines, stores a value in a global, and returns it if negative.
- `search_table_and_emit_entry`: Searches a table of byte-keyed entries for the key in the data register, returning an error code when no entry matches.
- `set_disk_read_configuration`: Stores a sequence of disk-controller values, including a zero, a length value, a sync pattern and control words, into consecutive global cells.
- `store_disk_length_config`: Stores three fixed values into consecutive global cells, forming a disk length configuration, and returns.
- `copy_sectors_after_seek`: Compares a passed value against a stored one and returns it when they match.
- `read_disk_track_sectors`: Waits for the disk controller, issues a sync-word read, then searches for the sync pattern and decodes each sector's odd and even bit fields into the buffer.
- `check_and_read_disk_sectors`: Returns an error when a count is zero.
- `seek_to_target_position`: If the short at a1 is negative it polls the CIA port A register until bit 4 is set, then clears that location.
- `set_port_state_code`: Calls a other routine, then writes one of two fixed state codes into a global cell depending on a status bit, calling other routines around each write.
- `return_status_if_flag_set`: When one bit of a status cell is set, it writes a fixed code, calls a other routine and returns zero.
- `store_state_code_and_call`: Stores a fixed state code in a global cell, calls another routine, and returns zero.
- `store_code_when_status_set`: When one bit of a status cell is set, it writes a second fixed state code, calls a other routine and returns zero.
- `write_port_status_byte`: Stores a constant byte into a global variable and then calls another routine.
- `wait_for_interrupt_flag`: Polls a status cell until one bit is set, then writes that bit back to clear it.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.

### logged disk load from Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf boot-block load

- `run_loader_state_machine`: Clears a block of longwords at a fixed address, then runs a loop that calls other routines and dispatches on a state value.
- `wait_while_status_nonnegative`: Calls a other routine repeatedly while a status cell is non-negative, then returns.
- `scan_string_then_call_library`: Calls the library routine at offset 240, then scans a string from a fixed offset to its terminating zero and calls the library routine at offset 60.
- `derive_state_from_counter`: Takes the top two bits of a counter cell as a state value, scales it, and writes the result into four cells at fixed offsets.
- `copy_four_bytes_from_table`: Copies four bytes from a table at an index-scaled offset into the destination buffer that the address register points to.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_tst_bne_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then returns.
- `frag_btst_bne_rte`: Short code fragment that tests bit 4 of a fixed memory address, then branches if the last result was not zero, then tests bit 5 of a fixed memory address, then branches if the last result was not zero, then tests bit 6 of a fixed memory address, then branches if the last result was not zero, then returns from the exception.
- `wait_for_interrupt_flag_2`: Polls the interrupt request register until one bit is set, then writes the same bit back to acknowledge it.
- `frag_clr_move_rts`: Short code fragment that clears a fixed memory address (2 times in a row), then copies 4178 to a fixed memory address, then copies 252 to a fixed memory address, then copies 13 to a fixed memory address, then copies 1 to a fixed memory address, then copies 130 to a fixed memory address, then copies 32776 to a fixed memory address, then returns.
- `frag_clr_move_rts_2`: Short code fragment that clears a fixed memory address, then copies 50332 to d0, then copies d0 to a fixed memory address (2 times in a row), then copies 10249 to a fixed memory address, then copies 9 to a fixed memory address, then copies 2 to a fixed memory address, then clears a fixed memory address (2 times in a row), then returns.
- `handle_cia_interrupt`: Clears the CIAA control register, reads the CIAA interrupt request register, and if bit 3 is set, calls another routine; otherwise, it checks bit 1 of the interrupt register to either increment a global variable or perform a series of conditional checks and calls to other routines before setting the custom interrupt request bit 3 and returning.
- `frag_cmpi_ble_bgt_addq_bra`: Short code fragment that compares a fixed memory address with 130, then branches if the comparison was less or equal, then compares a fixed memory address with 130, then branches if the comparison was greater, then adds 1 to the memory at a0, then jumps to another point in the code.
- `frag_clr_bsr_movem_move_rte`: Short code fragment that clears the memory at a0, then clears a fixed memory address, then calls another routine, then clears a fixed memory address, then restores saved registers from the stack, then copies 8 to a fixed memory address, then returns from the exception.
- `frag_addq_bne_move_rts`: Short code fragment that adds 1 to the memory at a0, then branches if the last result was not zero, then copies 156 to the memory at a0, then adds 1 to the memory just below a0 (moving a0 down), then branches if the last result was not zero, then copies 196 to the memory at a0, then adds 1 to the memory just below a0 (moving a0 down), then returns.
- `frag_clr_move_bra`: Short code fragment that clears a fixed memory address, then copies 65280 to a fixed memory address, then jumps to another point in the code.
- `process_status_flags`: Tests three specific memory locations and branches based on their values; if all three are zero, it loads a word from a memory offset, subtracts 32768, performs a signed division by 7, and if the result is non-zero, stores it in a global variable before returning.
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
