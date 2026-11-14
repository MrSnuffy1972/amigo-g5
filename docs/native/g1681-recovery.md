# Game 1681 (super hang on (1988)(activision)[cr defjam - ccs][t +2 band]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01681). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 1 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 23 of 23 named (0 author symbols), 23 of 23 described (1 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 1024 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 662 of 662 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1475 |
| code | 2266 |
| data | 891235 |
| unused | 6144 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf:bootblock | 1 | 46 | 2 |
| logged disk load from Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf boot-block load | 1 | 469 | 16 |
| logged disk load from Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf boot-block load | 1 | 147 | 5 |
| Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf bytes 1024-2048 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- super hang on (1988)(activision)[cr defjam - ccs][t +2 band]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf: custom-format (non-DOS) disk, 1746 non-empty blocks: matched against track-loaded segments
- super hang on (1988)(activision)[cr defjam - ccs][t +2 band]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf: the boot block reads 2048 bytes from disk offset 11264 to an address (found statically); recovered as a unit
- super hang on (1988)(activision)[cr defjam - ccs][t +2 band]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf: the boot block reads 3584 bytes from disk offset 2048 to an address (found statically); recovered as a unit
- super hang on (1988)(activision)[cr defjam - ccs][t +2 band]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf: bytes 1024-2048 look like code no run loaded: recovered position-independently (load address unknown)
- super hang on (1988)(activision)[cr defjam - ccs][t +2 band]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf: 3 logged loads cover 6656 bytes of the image
- super hang on (1988)(activision)[cr defjam - ccs][t +2 band]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf: blocks 11-21 (bytes 5632-11264) hold ASCII text
- super hang on (1988)(activision)[cr defjam - ccs][t +2 band]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf: blocks 33-341 (bytes 16896-175104) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.54)
- super hang on (1988)(activision)[cr defjam - ccs][t +2 band]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf: blocks 343-569 (bytes 175616-291840) outside the filesystem hold data, not code (opcode density 0.010, entropy 5.27)
- super hang on (1988)(activision)[cr defjam - ccs][t +2 band]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf: blocks 571-640 (bytes 292352-328192) outside the filesystem hold data, not code (opcode density 0.048, entropy 6.31)
- super hang on (1988)(activision)[cr defjam - ccs][t +2 band]/Super Hang On (1988)(Activision)[cr Defjam - CCS][t +2 Band].adf: blocks 644-1759 (bytes 329728-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.74)
- 1 never-loaded region(s), 1024 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

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
- `verify_boot_name_bytes`: Calls two other routines and stores a fixed byte in a global cell, then walks eight bytes of that cell, returning a negative error code on any mismatch.
- `search_table_and_emit_entry`: Searches a table of byte-keyed entries for the key in the data register, returning an error code when no entry matches.
- `set_disk_read_configuration`: Stores a sequence of disk-controller values, including a zero, a length value, a sync pattern and control words, into consecutive global cells.
- `store_disk_length_config`: Stores three fixed values into consecutive global cells, forming a disk length configuration, and returns.
- `copy_disk_sector_from_buffer`: Checks a block number and, if it differs from the current one, calls a other routine to load it.
- `read_disk_track_sectors`: Waits for the disk controller, issues a sync-word read, then searches for the sync pattern and decodes each sector's odd and even bit fields into the buffer.
- `check_and_read_disk_sectors`: Returns an error when a count is zero.
- `step_value_toward_target`: Compares an incoming value with a stored value and calls other routines to step the stored value toward it one unit at a time.
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
