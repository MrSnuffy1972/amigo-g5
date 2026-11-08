# Game 1312 (super hang on (1988)(activision)[cr defjam - ccs]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01312). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 1 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 16 of 16 named (0 author symbols), 16 of 16 described (0 by rule, 16 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 504 of 504 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 735 |
| code | 1652 |
| data | 892589 |
| unused | 6144 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Super Hang On (1988)(Activision)[cr Defjam - CCS].adf:bootblock | 1 | 40 | 2 |
| logged disk load from Super Hang On (1988)(Activision)[cr Defjam - CCS].adf boot-block load | 1 | 0 | 0 |
| logged disk load from Super Hang On (1988)(Activision)[cr Defjam - CCS].adf boot-block load | 1 | 464 | 14 |

## Cases handled

- super hang on (1988)(activision)[cr defjam - ccs]/Super Hang On (1988)(Activision)[cr Defjam - CCS].adf: custom-format (non-DOS) disk, 1746 non-empty blocks: matched against track-loaded segments
- super hang on (1988)(activision)[cr defjam - ccs]/Super Hang On (1988)(Activision)[cr Defjam - CCS].adf: the boot block reads 1024 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- super hang on (1988)(activision)[cr defjam - ccs]/Super Hang On (1988)(Activision)[cr Defjam - CCS].adf: the boot block reads 2048 bytes from disk offset 11264 to an address (found statically); recovered as a unit
- super hang on (1988)(activision)[cr defjam - ccs]/Super Hang On (1988)(Activision)[cr Defjam - CCS].adf: 2 logged loads cover 3072 bytes of the image
- super hang on (1988)(activision)[cr defjam - ccs]/Super Hang On (1988)(Activision)[cr Defjam - CCS].adf: blocks 4-21 (bytes 2048-11264) hold ASCII text
- super hang on (1988)(activision)[cr defjam - ccs]/Super Hang On (1988)(Activision)[cr Defjam - CCS].adf: blocks 33-341 (bytes 16896-175104) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.54)
- super hang on (1988)(activision)[cr defjam - ccs]/Super Hang On (1988)(Activision)[cr Defjam - CCS].adf: blocks 343-569 (bytes 175616-291840) outside the filesystem hold data, not code (opcode density 0.010, entropy 5.27)
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
