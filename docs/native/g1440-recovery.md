# Game 1440 (anarchy (1991)(psygnosis)[cr defjam - ccs][t +7 defjam - ccs]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01440). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903888 top-level bytes in 46 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 33 of 33 named (0 author symbols), 33 of 33 described (4 by rule, 5 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 456 of 456 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 940 of 940 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 52364 |
| code | 3234 |
| data | 773860 |
| padding | 10430 |
| unused | 64000 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| type | 2 | 117 | 2 |
| Anarchy (1991)(Psygnosis)[cr Defjam - CCS][t +7 Defjam - CCS].adf:Anarchy | 1 | 99 | 3 |
| Anarchy (1991)(Psygnosis)[cr Defjam - CCS][t +7 Defjam - CCS].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 260 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 454 | 23 |

## Cases handled

- anarchy (1991)(psygnosis)[cr defjam - ccs][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][t +7 Defjam - CCS].adf:DEFJAM_23: high-entropy data file (7.81 bits/byte), classified as packed data
- anarchy (1991)(psygnosis)[cr defjam - ccs][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][t +7 Defjam - CCS].adf:DEFJAM_24: high-entropy data file (7.74 bits/byte), classified as packed data
- anarchy (1991)(psygnosis)[cr defjam - ccs][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][t +7 Defjam - CCS].adf:DEFJAM_25: high-entropy data file (7.76 bits/byte), classified as packed data
- anarchy (1991)(psygnosis)[cr defjam - ccs][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][t +7 Defjam - CCS].adf:DEFJAM_05: high-entropy data file (7.88 bits/byte), classified as packed data
- anarchy (1991)(psygnosis)[cr defjam - ccs][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][t +7 Defjam - CCS].adf:DEFJAM_06: high-entropy data file (7.81 bits/byte), classified as packed data
- anarchy (1991)(psygnosis)[cr defjam - ccs][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][t +7 Defjam - CCS].adf:DEFJAM_07: high-entropy data file (7.87 bits/byte), classified as packed data
- anarchy (1991)(psygnosis)[cr defjam - ccs][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][t +7 Defjam - CCS].adf:DEFJAM_08: high-entropy data file (7.91 bits/byte), classified as packed data
- anarchy (1991)(psygnosis)[cr defjam - ccs][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][t +7 Defjam - CCS].adf:DEFJAM_10: high-entropy data file (7.82 bits/byte), classified as packed data
- anarchy (1991)(psygnosis)[cr defjam - ccs][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][t +7 Defjam - CCS].adf:DEFJAM_12: high-entropy data file (7.85 bits/byte), classified as packed data
- anarchy (1991)(psygnosis)[cr defjam - ccs][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][t +7 Defjam - CCS].adf:DEFJAM_0A: high-entropy data file (7.89 bits/byte), classified as packed data
- anarchy (1991)(psygnosis)[cr defjam - ccs][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][t +7 Defjam - CCS].adf:DEFJAM_14: high-entropy data file (7.87 bits/byte), classified as packed data
- anarchy (1991)(psygnosis)[cr defjam - ccs][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][t +7 Defjam - CCS].adf:DEFJAM_0C: high-entropy data file (7.84 bits/byte), classified as packed data
- anarchy (1991)(psygnosis)[cr defjam - ccs][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][t +7 Defjam - CCS].adf:DEFJAM_16: high-entropy data file (7.76 bits/byte), classified as packed data
- anarchy (1991)(psygnosis)[cr defjam - ccs][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][t +7 Defjam - CCS].adf:DEFJAM_0E: high-entropy data file (7.84 bits/byte), classified as packed data
- anarchy (1991)(psygnosis)[cr defjam - ccs][t +7 defjam - ccs]/Anarchy (1991)(Psygnosis)[cr Defjam - CCS][t +7 Defjam - CCS].adf:DEFJAM_18: high-entropy data file (7.84 bits/byte), classified as packed data
- runtime segment an address (play-long-keys): 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 352 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- `init_display_and_load_palette` (runtime segment an address): Calls the exec forbid routine, clears a block of longwords, and copies a table to low memory. Kind: disk hardware/track check. Pass path: the code continues to the palette loading and display initialization logic following the loop/check.

## Functions


### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_longwords_then_call`: If the destination pointer passed on the stack is non-null, copies sixteen longwords from a fixed offset of a base register into it.

### Anarchy (1991)(Psygnosis)[cr Defjam - CCS][t +7 Defjam - CCS].adf:Anarchy

- `unpack_compressed_program_entry`: Program entry that first copies a block of data into memory, then unpacks a bit-packed stream backward using literal bytes and back-references.
- `read_store_previous_longword`: Reads the longword just before an address register's position, stores its low byte through a pointer, and returns it shifted right by one bit with the top bit set.
- `advance_bit_buffer_by_count`: Shifts a bit buffer right by a count taken from a register, calling an unnamed refill routine each time the buffer empties.

### Anarchy (1991)(Psygnosis)[cr Defjam - CCS][t +7 Defjam - CCS].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_sprites_each_frame`: Advances a frame counter and recomputes sprite data pointers and sprite control words from stored coordinates.
- `dispatch_through_pointer_table`: Calls a routine whose address is read from a fixed offset in a pointer table held in memory.
- `call_pointer_if_flagged`: Checks if a byte in memory is non-zero before calling a routine through a function pointer read from a fixed offset in a table.
- `initialize_display_and_wait_for_input`: Initializes copper, display window, and sprite registers, clears certain memory ranges, and enters a loop that waits for a keyboard press before proceeding to further hardware or memory resets.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_display_and_load_palette`: Calls the exec forbid routine, clears a block of longwords, and copies a table to low memory.
- `write_register_words_to_global`: Writes eight 16-bit values taken from two input registers to one global location, in the order the code gives.
- `pause_with_delay_loops`: Runs six passes, each calling one unnamed routine and then a long countdown loop, which together make a timed pause.
- `execute_nested_delay_loops`: Executes a series of nested loops that perform decrementing arithmetic to create a timing delay.
- `build_plane_copper_entries`: Writes a copper header with a display control word derived from a count register, then writes one bitplane pointer move pair per plane.
- `build_sprite_copper_entries`: Writes eight sprite pointer move pairs into a copper list, one per sprite, taking each 32-bit pointer from a source table.
- `set_display_window_registers`: Writes four display window and data fetch start and stop values, then clears four bitplane control and modulo words, all through one base register.
- `save_interrupt_state_and_enable`: Stores the current interrupt-enable value in two saved slots, installs a routine as the handler, and writes a fixed enable mask to the interrupt control register.
- `disable_master_interrupts`: Writes a fixed value to the interrupt enable register with the set bit clear, which disables the master interrupt switch.
- `tiny_ori_movea`: Small routine of 2 instructions that merges 0 into d0, then copies d0 to a7.
- `decrement_counter_then_clear_requests`: Decrements a counter if it is nonzero, calls two unnamed routines, then writes a fixed value that clears three audio request bits in the interrupt request register.
- `clear_global_and_call_repeatedly`: Writes zero to a global and then calls one unnamed routine four times in sequence.
- `fill_seven_byte_records`: Fills a fixed number of seven-byte records, each holding bytes derived from a running value and an unnamed routine's result, a zero byte, and a longword taken from a register.
- `mix_seed_from_raster_position`: Updates a seed longword by XOR and addition with a table value and a constant, mixing in a value read from the vertical position register.
- `tiny_ori`: Small routine of 1 instruction that merges 120 into d0.
- `add_strided_table_offsets`: Adds 4, 3, 2 and 1 in turn to bytes at intervals of eight bytes across four separate tables, each with a fixed entry count.
- `read_keyboard_serial_with_delay`: When a delay counter is zero, reads the keyboard serial byte, inverts it, shifts it into a stored value, and reloads the counter with a long delay when the low bit is clear.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `dispatch_pending_key_code`: Acts on a pending key value using a per-entry type table: one type writes several words and another toggles a flag while calling three unnamed routines.
- `set_three_constant_globals`: Writes three constant values to globals, and the pseudo-code shows nothing more.
- `emit_string_with_escapes`: Walks a zero-terminated byte string, calling an unnamed routine for each character.
- `copy_table_row_by_character`: Maps a zero character to a letter O, searches a fixed character list for the input, then copies seven bytes spaced 44 bytes apart from the table entry at the matching index into a destination.
- `copy_seven_strided_bytes`: Copies seven bytes from a source at intervals of 44 bytes to a destination at the same intervals.
