# Game 2397 (total football (1996)(domark)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02397). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2703360 top-level bytes in 40 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 17 of 17 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 50 of 50 named (0 author symbols), 50 of 50 described (1 by rule, 49 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 21 of 21 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 1187437 bytes, 5 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2740 of 3028 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 998940 |
| code | 9080 |
| data | 1342682 |
| padding | 8594 |
| unused | 344064 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Total Football (1996)(Domark)(Disk 1 of 3).adf:boot | 1 | 690 | 44 |
| Total Football (1996)(Domark)(Disk 1 of 3).adf:bootblock | 1 | 18 | 1 |
| Total Football (1996)(Domark)(Disk 1 of 3).adf:player1.spr (raw code file, load address unknown) | 1 | 0 | 0 |
| Total Football (1996)(Domark)(Disk 1 of 3).adf:fastram.exe (raw code file, load address unknown) | 1 | 59 | 1 |
| Total Football (1996)(Domark)(Disk 1 of 3).adf:player2.spr (raw code file, load address unknown) | 1 | 0 | 0 |
| Total Football (1996)(Domark)(Disk 1 of 3).adf:player0.spr (raw code file, load address unknown) | 1 | 0 | 0 |
| Total Football (1996)(Domark)(Disk 1 of 3).adf:chipram.exe (raw code file, load address unknown) | 1 | 2261 | 4 |
| Total Football (1996)(Domark)(Disk 2 of 3).adf:keeper2.spr (raw code file, load address unknown) | 1 | 0 | 0 |
| Total Football (1996)(Domark)(Disk 2 of 3).adf:keeper0.spr (raw code file, load address unknown) | 1 | 0 | 0 |
| Total Football (1996)(Domark)(Disk 2 of 3).adf:happy2.spr (raw code file, load address unknown) | 1 | 0 | 0 |
| Total Football (1996)(Domark)(Disk 2 of 3).adf:happy0.spr (raw code file, load address unknown) | 1 | 0 | 0 |
| Total Football (1996)(Domark)(Disk 2 of 3).adf:stadium2.blk (raw code file, load address unknown) | 1 | 0 | 0 |
| Total Football (1996)(Domark)(Disk 2 of 3).adf:keeper1.spr (raw code file, load address unknown) | 1 | 0 | 0 |
| Total Football (1996)(Domark)(Disk 2 of 3).adf:happy1.spr (raw code file, load address unknown) | 1 | 0 | 0 |
| Total Football (1996)(Domark)(Disk 3 of 3).adf:player1.spr (raw code file, load address unknown) | 1 | 0 | 0 |
| Total Football (1996)(Domark)(Disk 3 of 3).adf:player2.spr (raw code file, load address unknown) | 1 | 0 | 0 |
| Total Football (1996)(Domark)(Disk 3 of 3).adf:player0.spr (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- total football (1996)(domark)(disk 1 of 3)/Total Football (1996)(Domark)(Disk 1 of 3).adf:player1.spr: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- total football (1996)(domark)(disk 1 of 3)/Total Football (1996)(Domark)(Disk 1 of 3).adf:fastram.exe: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- total football (1996)(domark)(disk 1 of 3)/Total Football (1996)(Domark)(Disk 1 of 3).adf:player2.spr: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- total football (1996)(domark)(disk 1 of 3)/Total Football (1996)(Domark)(Disk 1 of 3).adf:player0.spr: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- total football (1996)(domark)(disk 1 of 3)/Total Football (1996)(Domark)(Disk 1 of 3).adf:chipram.exe: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- total football (1996)(domark)(disk 2 of 3)/Total Football (1996)(Domark)(Disk 2 of 3).adf:keeper2.spr: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- total football (1996)(domark)(disk 2 of 3)/Total Football (1996)(Domark)(Disk 2 of 3).adf:keeper0.spr: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- total football (1996)(domark)(disk 2 of 3)/Total Football (1996)(Domark)(Disk 2 of 3).adf:happy2.spr: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- total football (1996)(domark)(disk 2 of 3)/Total Football (1996)(Domark)(Disk 2 of 3).adf:happy0.spr: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- total football (1996)(domark)(disk 2 of 3)/Total Football (1996)(Domark)(Disk 2 of 3).adf:stadium2.blk: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- total football (1996)(domark)(disk 2 of 3)/Total Football (1996)(Domark)(Disk 2 of 3).adf:keeper1.spr: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- total football (1996)(domark)(disk 2 of 3)/Total Football (1996)(Domark)(Disk 2 of 3).adf:happy1.spr: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- total football (1996)(domark)(disk 3 of 3)/Total Football (1996)(Domark)(Disk 3 of 3).adf:player1.spr: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- total football (1996)(domark)(disk 3 of 3)/Total Football (1996)(Domark)(Disk 3 of 3).adf:player2.spr: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- total football (1996)(domark)(disk 3 of 3)/Total Football (1996)(Domark)(Disk 3 of 3).adf:player0.spr: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- total football (1996)(domark)(disk 2 of 3)/Total Football (1996)(Domark)(Disk 2 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- total football (1996)(domark)(disk 3 of 3)/Total Football (1996)(Domark)(Disk 3 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- 15 never-loaded region(s), 1187437 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Total Football (1996)(Domark)(Disk 1 of 3).adf:boot

- `execute_entry_point`: An entry point routine that calls another routine.
- `configure_hardware_and_memory`: Modifies color, DMA, and interrupt registers while initializing a memory table through several subroutine calls and data writes.
- `initialize_memory_table`: Sets a global variable, writes constants to two memory locations and a byte location, calls another routine three times, and then copies 128 longwords from one table to another.
- `poll_bit_and_clear`: Sets a bit in a structure, polls a bit in another location, and then clears a bit while calling other routines.
- `set_clear_byte_flags`: Computes a byte address from an uninitialized address register plus a constant offset, sets four bits in that byte and clears two others, leaving the remaining bits unchanged.
- `set_clear_byte_bits`: Computes a byte address from an uninitialized address register plus a constant offset, sets two bits in that byte, clears one bit and sets it again, leaving the other bits unchanged.
- `divide_index_and_compare`: Divides a value in a data register by eleven, compares the quotient's low word against a word in a structure pointed to by an address register, and if they differ, calls another routine with a value combining the remainder and quotient before calling a second routine with no arguments.
- `synchronize_value_loop`: Iterates through a loop to match a value by setting or clearing bits in a structure, then writes several bytes to different offsets and waits for a bit to be set.
- `write_memory_offsets`: Writes byte values to four specific offsets relative to a register and waits for a bit in one of those locations to be set before clearing a bit in another.
- `set_flags_and_wait_for_bit_2`: Writes four constant byte values to specific offsets within a structure pointed to by a register, polls a bit in one offset until it is set, and then clears a different bit in another offset before returning.
- `initialize_sprite_state`: The routine writes several fields of a structure pointed to by a register, overwriting some fields multiple times, then waits in a loop until a flag bit in the structure is set, and finally writes two more fields before returning.
- `verify_and_transform_data`: Searches a table for a specific marker, performs bitwise transformations on data blocks, and uses a checksum to validate the result, potentially retrying the process.
- `compare_and_copy_string`: Compares a null-terminated string against a target buffer using case-insensitive logic.
- `modify_byte_bitmask`: Computes a byte address from an uninitialized address register plus a constant offset, sets four bits in that byte, and clears two others while leaving the remaining bits unchanged.
- `set_clear_byte_bits_2`: Computes a byte address from an input address register plus a constant offset, sets two bits in that byte, clears one bit and sets it again, leaving the other bits unchanged.
- `match_string_and_copy_data`: Compares a null-terminated string against a target buffer using case-insensitive logic.
- `compare_and_copy_string_data`: Compares a case-insensitive string against a target buffer and, upon a match, performs a segmented memory copy of data structures using calculated offsets and bitwise shifts.
- `write_value_to_pointer`: Writes a constant value to a location determined by dereferencing a pointer stored within a structure.
- `divide_and_validate_index`: Divides a value in a data register by eleven and compares the quotient to a word in a structure.
- `update_status_bitmask`: Iteratively modifies bits in a byte via a pointer based on a comparison between a register value and a shifted word from a structure.
- `process_flag_loop`: Manipulates bits in a byte through a loop, calls another routine to compare values, and performs a series of memory writes and a wait loop if a condition is met.
- `update_and_wait_for_flag`: Stores a word into a structure, checks a comparison, and if not equal, writes constant bytes to a structure and waits for a bit to be set.
- `clear_flags_and_wait_2`: Writes constant values to several byte locations referenced through a base pointer, then spins until a specific flag bit in one is set.
- `clear_flags_and_wait`: The routine writes constant values to several byte locations referenced through a base pointer, then spins until a specific flag bit in one of those locations becomes set, and finally clears a bit in another location before returning.
- `clear_status_word`: Calls another routine and then stores zero into a word at an offset from a pointer.
- `initialize_sprite_state_2`: Writes several fields to a structure pointed to by a register, overwriting some fields multiple times, then waits in a loop until a flag bit in the structure is set, and finally writes two more fields before returning.
- `verify_data_integrity`: Searches for a specific marker in a data stream, performs bitwise transformations on data, and validates the result against a checksum, retrying or failing based on the outcome.
- `perform_checksum_validation`: Iterates through data blocks to perform bitwise operations and XOR-based checksumming, retrying with different parameters if the validation fails.
- `validate_input_string`: Calculates a checksum or hash from an input buffer and compares it against a target string using case-insensitive matching.
- `modify_byte_bitmask_2`: Calculates a byte address from an uninitialized address register plus an offset and applies a bitwise OR with a mask followed by a bitwise AND with a mask.
- `modify_byte_bit_pattern`: Computes a byte address from an input address register plus a constant offset and modifies specific bits within that byte by setting two bits, clearing one, and resetting it.
- `validate_and_copy_input_string`: Calculates a checksum or hash from an input buffer and compares it against a target string using case-insensitive matching.
- `compare_and_copy_string_data_2`: Compares a case-insensitive string from a data buffer against a target string.
- `store_indirect_value`: Stores a longword into a memory location pointed to by a value found within a structure.
- `divide_and_validate_index_2`: Divides a value in a data register by eleven and compares the quotient against a word in a structure.
- `update_status_bitmask_and_wait`: Modifies specific bits in a status byte via a pointer based on a comparison between two values.
- `poll_and_update_status`: Modifies bits in a memory location through a pointer, performs a comparison, and if a condition is met, writes several constant bytes to a structure and waits for a specific bit to be set.
- `set_status_and_poll_bit`: Writes four constant byte values to specific offsets within a structure pointed to by a register, then enters a loop that waits for a specific bit to be set in one of those offsets.
- `set_flags_and_wait_for_bit`: Writes constant byte values to four offsets of a structure pointed to by a register, then spins until a specific bit in one of those bytes is set, and finally clears a bit in another byte before returning.
- `set_flags_and_poll_status`: Writes constant values to four memory locations relative to a base pointer, polls a specific bit in one location until it is set, and then clears a bit in another location before returning.
- `clear_structure_offset`: Stores a zero byte into a specific offset of a structure pointed to by a register.
- `initialize_sprite_state_3`: Writes several fields of a structure pointed to by a register, overwriting some fields multiple times, then waits in a loop until a flag bit in the structure is set, and finally writes two more fields before returning.
- `validate_checksum_data_2`: Searches for a specific marker in memory, performs bitwise transformations on data blocks, and validates the result against a checksum using a loop.
- `validate_checksum_data`: Performs bitwise XOR operations on data blocks to verify a checksum, retrying with different parameters and potentially triggering a failure routine if the validation fails.

### Total Football (1996)(Domark)(Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Total Football (1996)(Domark)(Disk 1 of 3).adf:fastram.exe (raw code file, load address unknown)

- `access_hardware_registers`: Interacts with CIA, DMA, interrupt, and color registers, including the keyboard serial register and joystick/mouse fire buttons.

### Total Football (1996)(Domark)(Disk 1 of 3).adf:chipram.exe (raw code file, load address unknown)

- `initialize_hardware_and_copy_data`: Configures hardware registers including CIA, DMA, and interrupt settings, then performs several memory block copies and clears specific memory regions.
- `update_hardware_registers`: Performs multiple word-sized writes to a memory block, which includes updating bitplane or color data and interacting with hardware registers like the DMA controller and CIAA.
- `sort_memory_table`: Iterates through a table of values and performs a sorting operation by swapping elements based on their size relative to others.
- `execute_indirect_call`: Calculates an offset from a global variable using a value in a register and performs an indirect call to the resulting address.
