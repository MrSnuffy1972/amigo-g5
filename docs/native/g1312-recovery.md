# Game 1312 (super hang on (1988)(activision)[cr defjam - ccs]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01312). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905664 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 10 raw-word lines |
| Functions named and described | pass, 41 of 41 named (0 author symbols), 41 of 41 described (11 by rule, 18 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 100 of 100 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 884 of 884 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2029 |
| code | 4642 |
| data | 892847 |
| padding | 2 |
| unused | 6144 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Super Hang On (1988)(Activision)[cr Defjam - CCS].adf:bootblock | 1 | 40 | 2 |
| logged disk load from Super Hang On (1988)(Activision)[cr Defjam - CCS].adf boot-block load | 1 | 0 | 0 |
| logged disk load from Super Hang On (1988)(Activision)[cr Defjam - CCS].adf boot-block load | 1 | 469 | 16 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 375 | 23 |

## Cases handled

- super hang on (1988)(activision)[cr defjam - ccs]/Super Hang On (1988)(Activision)[cr Defjam - CCS].adf: custom-format (non-DOS) disk, 1746 non-empty blocks: matched against track-loaded segments
- 100 executed addresses below an address (an address) run the game's own code, not harness stubs (their opcode words are not the fake OS's): treated as game code (LOWCODE, 15 Nov 2026)
- super hang on (1988)(activision)[cr defjam - ccs]/Super Hang On (1988)(Activision)[cr Defjam - CCS].adf: the boot block reads 1024 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- super hang on (1988)(activision)[cr defjam - ccs]/Super Hang On (1988)(Activision)[cr Defjam - CCS].adf: the boot block reads 2048 bytes from disk offset 11264 to an address (found statically); recovered as a unit
- runtime segment an address: 100 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- super hang on (1988)(activision)[cr defjam - ccs]/Super Hang On (1988)(Activision)[cr Defjam - CCS].adf: 2 logged loads cover 3072 bytes of the image
- super hang on (1988)(activision)[cr defjam - ccs]/Super Hang On (1988)(Activision)[cr Defjam - CCS].adf: 8 track-loaded blocks found in recovered runtime segments and classified through them
- super hang on (1988)(activision)[cr defjam - ccs]/Super Hang On (1988)(Activision)[cr Defjam - CCS].adf: blocks 4-21 (bytes 2048-11264) hold ASCII text
- super hang on (1988)(activision)[cr defjam - ccs]/Super Hang On (1988)(Activision)[cr Defjam - CCS].adf: blocks 33-341 (bytes 16896-175104) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.54)
- super hang on (1988)(activision)[cr defjam - ccs]/Super Hang On (1988)(Activision)[cr Defjam - CCS].adf: blocks 343-536 (bytes 175616-274944) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.25)
- super hang on (1988)(activision)[cr defjam - ccs]/Super Hang On (1988)(Activision)[cr Defjam - CCS].adf: blocks 539-549 (bytes 275968-281600) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.41)
- super hang on (1988)(activision)[cr defjam - ccs]/Super Hang On (1988)(Activision)[cr Defjam - CCS].adf: blocks 556-569 (bytes 284672-291840) outside the filesystem hold data, not code (opcode density 0.044, entropy 5.42)
- super hang on (1988)(activision)[cr defjam - ccs]/Super Hang On (1988)(Activision)[cr Defjam - CCS].adf: blocks 571-640 (bytes 292352-328192) outside the filesystem hold data, not code (opcode density 0.048, entropy 6.31)
- super hang on (1988)(activision)[cr defjam - ccs]/Super Hang On (1988)(Activision)[cr Defjam - CCS].adf: blocks 644-1759 (bytes 329728-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.74)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Super Hang On (1988)(Activision)[cr Defjam - CCS].adf:bootblock

- `setup_display_and_jump_2`: Programs the display controller registers with two different video modes, allocates a block of memory and calls the display handler, then installs a vertical-blank handler and triggers a software interrupt.
- `setup_display_and_jump`: Disables interrupts, configures the display controller registers, clears the interrupt request register, switches the user stack pointer to a stack at 384, and jumps to a fixed address.

### logged disk load from Super Hang On (1988)(Activision)[cr Defjam - CCS].adf boot-block load

- `load_disk_boot_sectors`: Validates a boot signature and searches for specific data markers in memory to identify disk chunks.
- `check_eight_byte_boot_signature`: Calls two unnamed routines, stores a value in a global, and returns it if negative.
- `transfer_sectors_from_table`: Searches a table of records for one whose first field matches a given value, then reads a byte and a word from the matching record and calls another routine to transfer sectors, repeating until the requested amount is handled.
- `init_disk_controller_registers`: Clears an interrupt request, writes fixed byte values to a CIA port B register, and stores the disk length, sync word, and DMA control values into their registers, then returns.
- `write_disk_drive_control_values`: Clears an interrupt request, writes three fixed byte values to a CIA port B register, and stores a value into the disk length register, then returns.
- `copy_sectors_after_seek`: If the requested position differs from the stored one it calls one unnamed routine and returns its error when negative.
- `process_disk_request_table`: If the requested position differs from the stored one it calls one unnamed routine and returns its error when negative.
- `validate_request_then_read_track`: Loads the value 249 into d0 and the stored position into d1, then tests bit 0 of d2.
- `seek_to_target_position`: If the short at a1 is negative it polls the CIA port A register until bit 4 is set, then clears that location.
- `select_control_and_call`: Calls one unnamed routine, then polls the CIA port A register until bit 4 is set.
- `check_ciaa_bit2_and_branch`: Tests bit 2 of the CIA port A register and returns immediately when that bit is clear, otherwise falling through into the following routine.
- `write_drive_control_and_call`: Writes the byte 112 then 117 to the CIA port B register at a fixed address, calls one unnamed routine, and returns zero.
- `check_ciaa_pra_bit2`: Reads bit 2 of the CIA peripheral register at address a fixed address and returns immediately when that bit is clear, otherwise it jumps to the routine at 1404.
- `write_alt_control_and_call`: Writes the byte 114 then 119 to the CIA port B register at a fixed address, calls one unnamed routine, and returns zero.
- `wait_for_interrupt_flag`: Polls an interrupt register until bit 5 is set, then clears that bit by writing to the interrupt request register, and returns.
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
- `calculate_division_result`: Performs arithmetic operations on a global variable, including subtraction, negation, and division by seven, then stores the result in another global variable and sets a bit in the condition codes register.
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
