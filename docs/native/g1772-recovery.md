# Game 1772 (cadaver (1991)(imageworks)(m3)[cr angels - genesis](disk 1 of 2)[t +4 full - evil]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01772). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902000 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 14 of 14 named (0 author symbols), 14 of 14 described (0 by rule, 11 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 131 of 131 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 148992 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 359 of 359 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 4575 |
| code | 1180 |
| data | 869106 |
| padding | 3 |
| unused | 27136 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Cadaver (1991)(ImageWorks)(M3)[cr Angels - Genesis](Disk 1 of 2)[t +4 Full - Evil].adf:bootblock | 1 | 5 | 1 |
| logged disk load from Cadaver (1991)(ImageWorks)(M3)[cr Angels - Genesis](Disk 1 of 2)[t +4 Full - Evil].adf boot-block load | 1 | 107 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 247 | 11 |
| Cadaver (1991)(ImageWorks)(M3)[cr Angels - Genesis](Disk 1 of 2)[t +4 Full - Evil].adf bytes 431104-505856 (code never loaded by a reference run) | 1 | 0 | 0 |
| Cadaver (1991)(ImageWorks)(M3)[cr Angels - Genesis](Disk 1 of 2)[t +4 Full - Evil].adf bytes 664576-738816 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- cadaver (1991)(imageworks)(m3)[cr angels - genesis](disk 1 of 2)[t +4 full - evil]/Cadaver (1991)(ImageWorks)(M3)[cr Angels - Genesis](Disk 1 of 2)[t +4 Full - Evil].adf: custom-format (non-DOS) disk, 1704 non-empty blocks: matched against track-loaded segments
- cadaver (1991)(imageworks)(m3)[cr angels - genesis](disk 1 of 2)[t +4 full - evil]/Cadaver (1991)(ImageWorks)(M3)[cr Angels - Genesis](Disk 1 of 2)[t +4 Full - Evil].adf: the boot block reads 5120 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 131 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- cadaver (1991)(imageworks)(m3)[cr angels - genesis](disk 1 of 2)[t +4 full - evil]/Cadaver (1991)(ImageWorks)(M3)[cr Angels - Genesis](Disk 1 of 2)[t +4 Full - Evil].adf: bytes 431104-505856 look like code no run loaded: recovered position-independently (load address unknown)
- cadaver (1991)(imageworks)(m3)[cr angels - genesis](disk 1 of 2)[t +4 full - evil]/Cadaver (1991)(ImageWorks)(M3)[cr Angels - Genesis](Disk 1 of 2)[t +4 Full - Evil].adf: bytes 664576-738816 look like code no run loaded: recovered position-independently (load address unknown)
- cadaver (1991)(imageworks)(m3)[cr angels - genesis](disk 1 of 2)[t +4 full - evil]/Cadaver (1991)(ImageWorks)(M3)[cr Angels - Genesis](Disk 1 of 2)[t +4 Full - Evil].adf: 3 logged loads cover 154112 bytes of the image
- cadaver (1991)(imageworks)(m3)[cr angels - genesis](disk 1 of 2)[t +4 full - evil]/Cadaver (1991)(ImageWorks)(M3)[cr Angels - Genesis](Disk 1 of 2)[t +4 Full - Evil].adf: blocks 12-37 (bytes 6144-19456) outside the filesystem hold data, not code (opcode density 0.042, entropy 4.97)
- cadaver (1991)(imageworks)(m3)[cr angels - genesis](disk 1 of 2)[t +4 full - evil]/Cadaver (1991)(ImageWorks)(M3)[cr Angels - Genesis](Disk 1 of 2)[t +4 Full - Evil].adf: blocks 39-82 (bytes 19968-42496) outside the filesystem hold data, not code (opcode density 0.019, entropy 5.12)
- cadaver (1991)(imageworks)(m3)[cr angels - genesis](disk 1 of 2)[t +4 full - evil]/Cadaver (1991)(ImageWorks)(M3)[cr Angels - Genesis](Disk 1 of 2)[t +4 Full - Evil].adf: blocks 84-747 (bytes 43008-382976) outside the filesystem hold data, not code (opcode density 0.021, entropy 7.07)
- cadaver (1991)(imageworks)(m3)[cr angels - genesis](disk 1 of 2)[t +4 full - evil]/Cadaver (1991)(ImageWorks)(M3)[cr Angels - Genesis](Disk 1 of 2)[t +4 Full - Evil].adf: blocks 764-840 (bytes 391168-430592) outside the filesystem hold data, not code (opcode density 0.028, entropy 6.93)
- cadaver (1991)(imageworks)(m3)[cr angels - genesis](disk 1 of 2)[t +4 full - evil]/Cadaver (1991)(ImageWorks)(M3)[cr Angels - Genesis](Disk 1 of 2)[t +4 Full - Evil].adf: blocks 1004-1296 (bytes 514048-664064) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.52)
- cadaver (1991)(imageworks)(m3)[cr angels - genesis](disk 1 of 2)[t +4 full - evil]/Cadaver (1991)(ImageWorks)(M3)[cr Angels - Genesis](Disk 1 of 2)[t +4 Full - Evil].adf: blocks 1459-1756 (bytes 747008-899584) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.53)
- cadaver (1991)(imageworks)(m3)[cr angels - genesis](disk 1 of 2)[t +4 full - evil]/Cadaver (1991)(ImageWorks)(M3)[cr Angels - Genesis](Disk 1 of 2)[t +4 Full - Evil].adf: blocks 1758-1759 (bytes 900096-901120) hold ASCII text
- 2 never-loaded region(s), 148992 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Cadaver (1991)(ImageWorks)(M3)[cr Angels - Genesis](Disk 1 of 2)[t +4 Full - Evil].adf:bootblock

- `copy_global_block_after_request`: Boot block entry that makes one library request through a stored pointer and then copies a block of bytes from a global area to a buffer until the destination reaches a limit.

### logged disk load from Cadaver (1991)(ImageWorks)(M3)[cr Angels - Genesis](Disk 1 of 2)[t +4 Full - Evil].adf boot-block load

- `shift_long_and_store_word`: Reads the longword four bytes before the address register, writes a word built from its low byte and the high byte of a data register through a pointer, and returns the value shifted right one bit with the top bit set.
- `shift_bits_through_refill_loop`: Shifts a bit register right one place per step for a count taken from its argument, calling a refill routine whenever the register empties.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_main_loop`: Repeatedly calls three subroutines within a loop until a condition is met, then performs several assignments to registers and memory before calling a function via a pointer.
- `wait_for_word_match`: The routine loops until the low word of a global variable, masked to nine bits, equals a word value in D0.
- `clear_memory_block`: Sets every longword in a specified memory range to zero.
- `process_table_character`: Calculates an offset into a table using a value from a structure in a5, fetches a byte from a0, and if the byte is not negative, subtracts 32 and calls another routine to load table values before looping.
- `load_table_values`: Casts an index value to a signed byte and stores a byte from a global table into the first byte of a destination buffer.
- `calculate_string_length`: Increments a counter while traversing a null-terminated string and stores a calculated value into a global variable.
- `update_index_and_pointer`: Checks a byte value and performs conditional loops to find a null terminator in a buffer, then updates a short value and a global variable with a calculated result.
- `decrement_and_wrap_counter`: Decrements a byte in a structure and, if it reaches zero, performs logic to increment or decrement a separate counter and update a status byte.
- `convert_string_to_digits`: Sets several values in a structure in a5, then loops through a sequence of bytes starting at an offset in a5, subtracting 48 from each byte and storing the result in another location in a5 until a zero byte is encountered, at which point it calls another routine.
- `update_display_counters`: Checks if a byte in d0 is 2, copies a byte from a0 to a4 and calls another routine, then enters a loop that updates several bytes at specific offsets in a structure in a5 using values derived from a 16-bit word at offset 32 of that structure, and finally calls another routine.
- `update_timer_display_string`: Decrements a counter and, upon reaching zero, resets specific status flags and updates a three-digit decimal string representation of a value.
