# Game 1680 (super hang on (1988)(activision)[cr defjam - ccs][m ela team - fbi soft]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01680). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 1 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 16 of 16 named (0 author symbols), 16 of 16 described (0 by rule, 0 by the local model) |
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
| Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf:bootblock | 1 | 40 | 2 |
| logged disk load from Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf boot-block load | 1 | 0 | 0 |
| logged disk load from Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf boot-block load | 1 | 464 | 14 |

## Cases handled

- super hang on (1988)(activision)[cr defjam - ccs][m ela team - fbi soft]/Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf: custom-format (non-DOS) disk, 1746 non-empty blocks: matched against track-loaded segments
- super hang on (1988)(activision)[cr defjam - ccs][m ela team - fbi soft]/Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf: the boot block reads 1024 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- super hang on (1988)(activision)[cr defjam - ccs][m ela team - fbi soft]/Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf: the boot block reads 2048 bytes from disk offset 11264 to an address (found statically); recovered as a unit
- super hang on (1988)(activision)[cr defjam - ccs][m ela team - fbi soft]/Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf: 2 logged loads cover 3072 bytes of the image
- super hang on (1988)(activision)[cr defjam - ccs][m ela team - fbi soft]/Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf: blocks 4-21 (bytes 2048-11264) hold ASCII text
- super hang on (1988)(activision)[cr defjam - ccs][m ela team - fbi soft]/Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf: blocks 33-341 (bytes 16896-175104) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.54)
- super hang on (1988)(activision)[cr defjam - ccs][m ela team - fbi soft]/Super Hang On (1988)(Activision)[cr Defjam - CCS][m ELA Team - FBI Soft].adf: blocks 343-569 (bytes 175616-291840) outside the filesystem hold data, not code (opcode density 0.010, entropy 5.27)
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
