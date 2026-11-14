# Game 1682 (super hang on (1988)(activision)[cr defjam - ccs][t +3 subway - dream team]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01682). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 1 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 50 of 50 named (0 author symbols), 50 of 50 described (5 by rule, 12 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2 of 2 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 2048 bytes, 10 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1223 of 1223 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1500 |
| code | 4084 |
| data | 889388 |
| padding | 4 |
| unused | 6144 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf:bootblock | 1 | 77 | 3 |
| logged disk load from Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf boot-block load | 1 | 290 | 22 |
| logged disk load from Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf boot-block load | 1 | 503 | 15 |
| Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf bytes 11264-11776 (code never loaded by a reference run) | 1 | 0 | 0 |
| Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf blocks 23-25 (bytes 11776-13312) (code never loaded by a reference run, load address unknown) | 1 | 353 | 10 |

## Cases handled

- super hang on (1988)(activision)[cr defjam - ccs][t +3 subway - dream team]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf: custom-format (non-DOS) disk, 1746 non-empty blocks: matched against track-loaded segments
- super hang on (1988)(activision)[cr defjam - ccs][t +3 subway - dream team]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf: the boot block reads 4608 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- super hang on (1988)(activision)[cr defjam - ccs][t +3 subway - dream team]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf: the boot block reads 2048 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- super hang on (1988)(activision)[cr defjam - ccs][t +3 subway - dream team]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf: bytes 11264-11776 look like code no run loaded: recovered position-independently (load address unknown)
- super hang on (1988)(activision)[cr defjam - ccs][t +3 subway - dream team]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf: 3 logged loads cover 7168 bytes of the image
- super hang on (1988)(activision)[cr defjam - ccs][t +3 subway - dream team]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf: blocks 15-21 (bytes 7680-11264) hold ASCII text
- Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf blocks 23-25 (bytes 11776-13312) (code never loaded by a reference run, load address unknown): never-loaded code-like bytes recovered position-independently (load address unknown, 1026 of 1536 bytes decode)
- super hang on (1988)(activision)[cr defjam - ccs][t +3 subway - dream team]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf: blocks 33-341 (bytes 16896-175104) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.54)
- super hang on (1988)(activision)[cr defjam - ccs][t +3 subway - dream team]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf: blocks 343-569 (bytes 175616-291840) outside the filesystem hold data, not code (opcode density 0.010, entropy 5.27)
- super hang on (1988)(activision)[cr defjam - ccs][t +3 subway - dream team]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf: blocks 571-640 (bytes 292352-328192) outside the filesystem hold data, not code (opcode density 0.048, entropy 6.31)
- super hang on (1988)(activision)[cr defjam - ccs][t +3 subway - dream team]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf: blocks 644-1759 (bytes 329728-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.74)
- 1 never-loaded region(s), 512 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `init_disk_controller_registers` (Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf boot-block load +an address to an address (2048 bytes)): Writes a fixed sequence of values to the disk length, sync word, ADKCON, DMA control and interrupt request registers. Kind: disk hardware/sync check. Pass path: the instruction following the bsr.w an address call (st.b an address.l).
- `write_disk_drive_control_values` (Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf boot-block load +an address to an address (2048 bytes)): Writes three fixed values to the CIA-B port B register, the disk length register and the DMA control register. Kind: disk hardware/parameter configuration. Pass path: the code continues to the next instruction after the routine returns (the branch at an address is likely the entry/exit logic).
- `read_mfm_sector_with_retries` (Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf boot-block load +an address to an address (2048 bytes)): Starts a disk read, searches the raw MFM stream for the sync word, and decodes each sector's odd and even bit planes into memory. Kind: checksum/data integrity check. Pass path: the loop continues or returns the original value (in_D1) if sVar3 reaches -1 without a checksum failure.
- `write_drive_control_and_call` (Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf boot-block load +an address to an address (2048 bytes)): Writes a fixed control value to the CIA-B port B drive register, makes one call, and returns zero. Kind: disk hardware/track check. Pass path: bra.b an address.
- `write_alt_control_and_call` (Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf boot-block load +an address to an address (2048 bytes)): Writes the second fixed control value to the CIA-B port B drive register, makes one call, and returns zero. Kind: disk hardware/checksum trap. Pass path: bra.b an address.
- `load_data_blocks` (Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf blocks 23-25 (bytes 11776-13312) (code never loaded by a reference run, load address unknown)): Loops to call another routine and potentially another routine to copy data, then clears the interrupt request register and writes specific values to the CIA port B and DMA control registers. Kind: disk hardware/sync check. Pass path: the loop continues to the next block/iteration.
- `process_disk_data_block` (Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf blocks 23-25 (bytes 11776-13312) (code never loaded by a reference run, load address unknown)): Checks a value against a global variable and calls another routine, then configures DMA and custom hardware registers before entering a nested loop that performs bitwise manipulations and XOR operations to write data from a source table to a destination buffer starting at a fixed address. Kind: bit-manipulation checksum/decryption trap. Pass path: the branch where sVar3 reaches -1 and returns in_D1.
- `write_drive_control_and_call_2` (Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf blocks 23-25 (bytes 11776-13312) (code never loaded by a reference run, load address unknown)): Writes the byte 112 then 117 to the CIA port B register at a fixed address, calls another routine, and returns zero. Kind: disk hardware/sync check. Pass path: bra.b an address.
- `write_alt_control_and_call_2` (Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf blocks 23-25 (bytes 11776-13312) (code never loaded by a reference run, load address unknown)): Writes the byte 114 then 119 to the CIA port B register at a fixed address, calls another routine, and returns zero. Kind: disk hardware/sync check. Pass path: bra.b an address.

## Functions


### Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf:bootblock

- `boot_block_allocmem_entry`: Boot-block entry that calls exec.AllocMem and makes two further unnamed calls.
- `set_display_color_and_dma`: Nine-instruction routine whose listed registers are the bitplane control, background colour, DMA control and interrupt enable registers.
- `fade_color_on_raster_lines`: Waits for raster lines through the vertical position register and, on every eighth line, steps a colour value down in each of its three 4-bit fields before writing it to the colour register.

### logged disk load from Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf boot-block load

- `open_graphics_and_setup_copper`: Opens the graphics library, clears a block of memory, and sets the copper pointers and DMA control register.
- `store_routine_addresses`: Loads thirteen routine addresses into a table and stores each longword into consecutive slots, then returns.
- `clear_word_store_three_bytes_3`: Clears a word in an object structure and writes three constant bytes to a fixed memory location.
- `clear_word_store_three_bytes_4`: Clears a word at a different object offset and writes three constant bytes to another fixed memory location.
- `clear_word_store_three_bytes_5`: Clears a word at another object offset and writes three constant bytes to a further fixed memory location.
- `set_word_flag_to_one`: Sets a word flag in an object structure to one and returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_return_stub`: Returns at once without doing any work.
- `set_flag_store_yes_text`: Sets a word flag in an object structure to one and writes three constant bytes that spell the ASCII text YES into a fixed memory location.
- `set_flag_store_yes_text_2`: Sets a word flag at another object offset to one and writes the same three ASCII bytes, YES, into another fixed memory location.
- `set_flag_store_yes_text_3`: Sets a word flag at another object offset to one and writes three constant bytes that spell the ASCII text YES into a further fixed memory location.
- `set_word_flag_to_one_2`: Sets a word flag in an object structure to one and returns.
- `empty_return_stub_2`: Returns at once without doing any work.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `handle_button_press_with_delay`: Counts down a delay and, once it expires, checks the fire-button bit of the CIA-A port register and the right-button bit of the POTGOR register.
- `update_mouse_position_from_delta`: Computes the change in the mouse movement byte since the last call, subtracts it from a stored position value, and clamps the value between zero and a stored maximum.
- `cycle_color_entry_values`: Sets a set of colour entries to white, copies a stored value into the entry chosen by an index, and then steps that stored value by a signed step.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `set_entries_by_parity_pattern`: Loops ten times over a table of offsets, setting each selected entry to one of two constant values depending on the parity of a stored byte, and makes one call per pass.
- `program_blitter_and_advance_counter`: Waits for the blitter to finish, writes a fixed set of words into its mask, control, modulo, pointer and size registers, then increments a counter.
- `draw_ascii_text_rows`: For each character in a string, takes the character code minus the ASCII space offset and selects its eight-row glyph from a table with a 192-byte stride.

### logged disk load from Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf boot-block load

- `check_status_then_compare_bytes`: Makes two calls and stores a value in a global, then returns the routine's result if it is negative.
- `find_entry_and_process_chunks`: Searches a table of up to 63 two-word entries for a matching key and returns a negative error code when none matches.
- `init_disk_controller_registers`: Writes a fixed sequence of values to the disk length, sync word, ADKCON, DMA control and interrupt request registers.
- `write_disk_drive_control_values`: Writes three fixed values to the CIA-B port B register, the disk length register and the DMA control register.
- `copy_sectors_after_seek`: Copies a run of 512-byte blocks from a fixed source region into a destination pointer.
- `read_mfm_sector_with_retries`: Starts a disk read, searches the raw MFM stream for the sync word, and decodes each sector's odd and even bit planes into memory.
- `validate_request_then_read_track`: Validates the request value and stores a fallback value in a global when a flag bit is clear, returning an error if the request is zero.
- `seek_to_target_position`: Compares a requested position with a stored one and makes one call for each unit of difference, updating the stored position as it goes.
- `select_drive_by_status_bit`: Tests a status bit of the CIA-A port register and writes one of two fixed control values depending on it, making calls around each write.
- `check_disk_status`: Tests bit 2 of the CIAA port A register and branches if it is not set.
- `write_drive_control_and_call`: Writes a fixed control value to the CIA-B port B drive register, makes one call, and returns zero.
- `check_disk_status_alt`: Tests bit 2 of the CIAA port A register and branches if it is not set.
- `write_alt_control_and_call`: Writes the second fixed control value to the CIA-B port B drive register, makes one call, and returns zero.
- `wait_for_interrupt_flag`: Polls a flag word until bit five is set, then writes that bit alone back to clear it.
- `wait_for_blitter_2`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.

### Super Hang On (1988)(Activision)[cr Defjam - CCS][t +3 Subway - Dream Team].adf blocks 23-25 (bytes 11776-13312) (code never loaded by a reference run, load address unknown)

- `load_data_blocks`: Loops to call another routine and potentially another routine to copy data, then clears the interrupt request register and writes specific values to the CIA port B and DMA control registers.
- `copy_sectors_after_seek_2`: Compares a passed value against a stored one and returns it when they match.
- `process_disk_data_block`: Checks a value against a global variable and calls another routine, then configures DMA and custom hardware registers before entering a nested loop that performs bitwise manipulations and XOR operations to write data from a source table to a destination buffer starting at a fixed address.
- `seek_to_target_position_2`: If the short at a1 is negative it polls the CIA port A register until bit 4 is set, then clears that location.
- `execute_drive_commands`: Calls another routine, then checks bit 4 of the CIAA port A register to decide whether to call two different sequences of other routines.
- `test_cia_status_bit`: Tests bit 2 of a hardware register and branches if the bit is not set.
- `write_drive_control_and_call_2`: Writes the byte 112 then 117 to the CIA port B register at a fixed address, calls another routine, and returns zero.
- `check_disk_status_short`: Tests bit 2 of the CIAA port A register and branches if it is not set.
- `write_alt_control_and_call_2`: Writes the byte 114 then 119 to the CIA port B register at a fixed address, calls another routine, and returns zero.
- `wait_vblank_bit5`: Waits until bit 5 of the VBLANK status register is set, then sets bit 5 of the DMA control register and returns; if that bit is not set it checks bit 6 of the input port register and returns without changing anything.
