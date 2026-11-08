# Game 508 (Heroes of the Lance V1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00508). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1812240 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 24 raw-word lines |
| Functions named and described | pass, 38 of 38 named (0 author symbols), 38 of 38 described (3 by rule, 35 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 393 of 393 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 7168 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 843 of 843 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 4801 |
| code | 3084 |
| data | 1787921 |
| padding | 50 |
| unused | 16384 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf:bootblock | 1 | 102 | 3 |
| logged disk load from Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 741 | 35 |
| Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf bytes 1536-8704 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- heroes of the lance v1.0 (1988)(ssi)(disk 1 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf: custom-format (non-DOS) disk, 1744 non-empty blocks: matched against track-loaded segments
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: custom-format (non-DOS) disk, 1740 non-empty blocks: matched against track-loaded segments
- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4E75) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- heroes of the lance v1.0 (1988)(ssi)(disk 1 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf: the boot block reads 512 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 299 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- heroes of the lance v1.0 (1988)(ssi)(disk 1 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf: bytes 1536-8704 look like code no run loaded: recovered position-independently (load address unknown)
- heroes of the lance v1.0 (1988)(ssi)(disk 1 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf: 2 logged loads cover 7680 bytes of the image
- heroes of the lance v1.0 (1988)(ssi)(disk 1 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf: 4 track-loaded blocks found in recovered runtime segments and classified through them
- heroes of the lance v1.0 (1988)(ssi)(disk 1 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf: blocks 23-157 (bytes 11776-80896) outside the filesystem hold data, not code (opcode density 0.042, entropy 6.88)
- heroes of the lance v1.0 (1988)(ssi)(disk 1 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf: blocks 159-159 (bytes 81408-81920) outside the filesystem hold data, not code (opcode density 0.031, entropy 1.37)
- heroes of the lance v1.0 (1988)(ssi)(disk 1 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf: blocks 161-806 (bytes 82432-413184) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.99)
- heroes of the lance v1.0 (1988)(ssi)(disk 1 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf: blocks 809-821 (bytes 414208-420864) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.16)
- heroes of the lance v1.0 (1988)(ssi)(disk 1 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf: blocks 825-879 (bytes 422400-450560) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.92)
- heroes of the lance v1.0 (1988)(ssi)(disk 1 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf: blocks 882-894 (bytes 451584-458240) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.16)
- heroes of the lance v1.0 (1988)(ssi)(disk 1 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf: blocks 898-1759 (bytes 459776-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.76)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 2-411 (bytes 1024-210944) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.26)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 413-425 (bytes 211456-218112) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.54)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 427-442 (bytes 218624-226816) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.31)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 444-768 (bytes 227328-393728) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.94)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 770-1159 (bytes 394240-593920) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.91)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 1161-1162 (bytes 594432-595456) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.64)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 1164-1169 (bytes 595968-599040) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.96)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 1171-1247 (bytes 599552-638976) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.43)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 1249-1344 (bytes 639488-688640) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.09)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 1346-1347 (bytes 689152-690176) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.92)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 1349-1360 (bytes 690688-696832) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.92)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 1362-1363 (bytes 697344-698368) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.45)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 1365-1366 (bytes 698880-699904) outside the filesystem hold data, not code (opcode density 0.002, entropy 1.90)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 1368-1378 (bytes 700416-706048) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.16)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 1381-1382 (bytes 707072-708096) outside the filesystem hold data, not code (opcode density 0.004, entropy 0.89)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 1384-1400 (bytes 708608-717312) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.36)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 1402-1403 (bytes 717824-718848) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.02)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 1405-1759 (bytes 719360-901120) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.03)
- 1 never-loaded region(s), 7168 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf:bootblock

- `find_and_init_io_request`: Initializes an I/O request in a structure, then searches a table of signatures until it finds a match and stores the matching entry's offsets back into that request.
- `send_zero_length_io_command`: Stores the decimal value nine at offset 28 of the structure pointed to by a1, clears the longword at offset 36 of the structure pointed to by a3, then calls the routine at offset -744 of the code referenced by a global variable and returns.
- `apply_relocation_table_offsets`: Loads a base offset from a global variable, adds it to each of three other global values, then reads a longword from a table and adds the base to every longword it addresses.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_and_run_io_requests`: Initializes the system and library environment, then calls a routine to set up an I/O request and invokes a routine pointer stored in a structure.
- `update_structure_values_2`: Updates several fields in a structure pointed to by a global variable using values from other offsets within that structure, calls a routine via a pointer, and may recursively call itself if a specific value is found at a global variable.
- `set_color00_and_loop`: Writes the decimal value 4095 to the COLOR00 hardware register and then branches back to itself, forming an infinite loop that does nothing else.
- `set_color00_and_loop_2`: Writes the decimal value 3840 to the COLOR00 hardware register and then branches back to itself, forming an infinite loop that does nothing else.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `call_indexed_routine`: Disables interrupts, shifts the value in register d0 left by two, adds it to a base address, and calls the routine located at the resulting address before returning from an interrupt.
- `set_bits_in_register`: Performs a bitwise OR on the value in register d0 with each of the listed constants in sequence, then returns without storing the result.
- `clear_memory_blocks`: Calculates a loop count from a 16-bit value in register d1 by multiplying by ten and subtracting one, then repeatedly clears longwords at offsets 6000, 4000, 2000 and zero of two global arrays until the counter reaches zero.
- `apply_clamped_rotated_bitmask_2`: Adjusts a rotation index and a vertical offset from a table and register values, clamping the offset, then applies a rotated bitmask from a table.
- `apply_clamped_rotated_bitmask`: Adjusts a rotation index and a vertical offset from a table and register values, clamping the offset, then applies a rotated bitmask from a source list of four 16-bit words to a destination buffer, skipping words whose bits are all set.
- `apply_rotated_bitmask_to_buffer`: Computes a bit mask from a byte read at an offset in a table, then loops over a source list of four 16-bit words, applying a rotated mask to each pair and OR-ing the result into the destination words pointed to by a2, repeating for a count derived from the table byte.
- `initialize_memory_range`: Stores an address, an address, and an address into three consecutive global variables, then clears a 32768-byte block starting at the third address by writing zero words in a loop that decrements from 8192.
- `configure_hardware_registers`: Writes fixed values to five hardware registers, then reads a register, sets bits 15 and 9 of it, and stores the result back, before loading a pointer into a video register and reading another register.
- `toggle_and_add_constant`: Toggles bit 0 of a global variable, then copies two words from a table into a structure and stores three incremented values plus a modified counter into consecutive fields.
- `transform_and_store_data`: Saves a pointer into a global variable, then reads a word from a list, doubles it, adds 273, and stores the result into consecutive words in a loop that runs 16 times.
- `process_index_table_copy_2_2`: Clamps a signed 16-bit value to the range 0 to 208, then stores it masked to 511 into a global variable.
- `process_index_table_copy_2`: Clamps an index to a maximum of 304, stores it masked to 511 into a global variable, then based on the low nibble either sets four destination pointers and copies sixteen bytes from a source buffer in four interleaved passes, or reads 16-bit values from the source and shifts them by a bit count derived from the index.
- `process_index_table_copy`: Clamps an index to a maximum of 304, stores it masked to 511 into a global variable, then based on the low nibble either sets four destination pointers and copies eight bytes per iteration, or shifts each of four 16-bit source words by a computed amount and stores them at fixed offsets, looping until the source and destination counters reach zero.
- `distribute_word_data`: Saves three registers, sets up four destination pointers, then copies four words from a source table into the destinations in an inner loop of 20 iterations while advancing the source offset.
- `distribute_byte_data`: Saves three registers, sets up four destination pointers, then copies eight bytes from a source table into the destinations in an inner loop of 20 iterations while advancing the source offset.
- `execute_disk_io`: Calls a helper routine, then fills a disk I/O structure with a command, a byte count, and two buffer addresses before invoking the DoIO function pointer and restoring the saved registers.
- `call_do_i_o`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `call_do_i_o_2`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `setup_io_request`: Fills a disk I/O structure with a command, a byte count, and two buffer addresses, then calls DoIO, invokes a helper routine, and reads a status byte from the structure.
- `compare_null_terminated_strings`: Compares two byte strings in a loop of 12 iterations, returning 65535 on a mismatch or when the counter underflows, otherwise it returns a 16-bit value built from the last two bytes read.
- `execute_indirect_calls`: Loads a pointer from a global variable, calls the routine at the address found by adding an offset of negative 330 to that pointer, and repeats this process with several other offsets and values stored in the same structure.
- `update_structure_values`: Stores the value at offset 40 of the structure in a1, sets the 16-bit value at offset 28 to 2, writes the 16-bit value at offset 12 multiplied by 512 to offset 44 and the 16-bit value at offset 14 multiplied by 512 to offset 36, then calls a routine via a pointer.
- `setup_io_request_scaled`: Fills a disk I/O structure with a command and two buffer sizes derived from a global variable multiplied by 512, then calls DoIO, invokes a helper routine, and reads a status byte from the structure.
- `process_offset_table`: Reads a 32-bit value from a calculated pointer and returns immediately if it is zero, otherwise it adds that value to each entry of a table in a global variable.
- `clear_io_request`: Writes the decimal 9 to offset 28 of a structure pointed to by a global variable and clears the 32-bit value at offset 36 of that structure.
- `process_io_sequence`: Calls a routine to store a value in a global variable, then calls another routine and checks the result; if the value is non-negative it stores that value in the global variable and calls a third routine, then calls a fourth routine to clear an I/O request before returning.
- `init_disk_io_table`: Saves the caller's a1 register, stores the a0 register into a global variable, calls a helper routine, and if a status flag is not set it clears a 511-byte table and writes the value 3 into a field at offset 6 of that table.
- `copy_string_and_setup_io`: Calls a routine to store a value in a global variable, and if that value is negative it copies a string from one location to another using a loop.
- `apply_font_mask_to_buffers`: Reads each character from a null-terminated string and applies the corresponding 8-byte font glyph to four screen buffers, using bitwise OR or AND depending on a flag bit, advancing by 8000-byte rows and jumping ahead by 360 bytes on a carriage return.
- `allocate_initial_memory`: Clears the DMA enable register, sets the stack pointer to a fixed location, and calls the memory allocation library twice to reserve two blocks, storing the returned addresses in a structure.
