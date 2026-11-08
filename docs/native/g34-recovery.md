# Game 34 (Bionic Commando): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00034). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902560 top-level bytes in 38 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 43 of 43 named (0 author symbols), 43 of 43 described (11 by rule, 29 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 559 of 559 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1245 of 1245 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 44300 |
| code | 4038 |
| data | 321224 |
| padding | 9222 |
| unused | 523776 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Bionic Commando.adf:DEFJAM | 2 | 117 | 2 |
| Bionic Commando.adf:TRIO | 2 | 228 | 11 |
| Bionic Commando.adf:DEFJAM_1 | 1 | 3 | 1 |
| Bionic Commando.adf:DEFJAM_2 | 3 | 417 | 9 |
| Bionic Commando.adf:DEFJAM_3 | 3 | 228 | 7 |
| Bionic Commando.adf:DEFJAM_4 | 2 | 117 | 2 |
| Bionic Commando.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 125 | 10 |

## Cases handled

- Bionic Commando/Bionic Commando.adf:level2.pi1: high-entropy data file (7.62 bits/byte), classified as packed data
- Bionic Commando/Bionic Commando.adf:bigsol.spr: high-entropy data file (7.60 bits/byte), classified as packed data
- Bionic Commando/Bionic Commando.adf:mansp.spr: high-entropy data file (7.75 bits/byte), classified as packed data
- Bionic Commando/Bionic Commando.adf:map2.dat: high-entropy data file (7.70 bits/byte), classified as packed data
- Bionic Commando/Bionic Commando.adf:tune4.dat: high-entropy data file (7.82 bits/byte), classified as packed data
- Bionic Commando/Bionic Commando.adf:level345.spr: high-entropy data file (7.70 bits/byte), classified as packed data
- Bionic Commando/Bionic Commando.adf:level5.pi1: high-entropy data file (7.65 bits/byte), classified as packed data
- Bionic Commando/Bionic Commando.adf:tune2.dat: high-entropy data file (7.81 bits/byte), classified as packed data
- Bionic Commando/Bionic Commando.adf:level3.pi1: high-entropy data file (7.69 bits/byte), classified as packed data
- Bionic Commando/Bionic Commando.adf:level1.pi1: high-entropy data file (7.72 bits/byte), classified as packed data
- Bionic Commando/Bionic Commando.adf:spic.pi1: high-entropy data file (7.64 bits/byte), classified as packed data
- Bionic Commando/Bionic Commando.adf:level3.spr: high-entropy data file (7.67 bits/byte), classified as packed data
- Bionic Commando/Bionic Commando.adf:tune5.dat: high-entropy data file (7.79 bits/byte), classified as packed data
- Bionic Commando/Bionic Commando.adf:map1.dat: high-entropy data file (7.71 bits/byte), classified as packed data
- Bionic Commando/Bionic Commando.adf:tune3.dat: high-entropy data file (7.82 bits/byte), classified as packed data
- Bionic Commando/Bionic Commando.adf:tune1.dat: high-entropy data file (7.79 bits/byte), classified as packed data
- runtime segment an address: 93 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Bionic Commando.adf:DEFJAM

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `transfer_register_and_branch`: Moves the contents of register d1 into register d0 and then performs a short branch to another routine.

### Bionic Commando.adf:TRIO

- `initialize_resource_table`: Verifies the presence of required libraries by checking for specific magic strings in the library headers and parses a resource table.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Bionic Commando.adf:DEFJAM_1

- `wait_for_ciaa_bit`: Polls bit 6 of the CIAA Port A Data Register in a loop until it is cleared.

### Bionic Commando.adf:DEFJAM_2

- `process_dos_hunk_table`: Walks a linked list of memory entries to find a specific hunk type, then allocates memory and copies a sequence of entries into a new buffer.
- `read_next_longword_from_stream`: Copies the longword at the current read pointer into a global variable and advances the pointer to the next entry.
- `manage_dynamic_memory_buffer`: Attempts to allocate memory using a specific size and flags, then either stores the resulting pointer into a table or, if allocation fails, sets a global flag and enters a loop to free a list of memory blocks.
- `close_library_and_permit`: Calls a library function to close a library if a global pointer is non-zero, calls another routine, and then calls a library function to re-enable multitasking before returning zero.
- `free_two_stored_buffers`: Frees each of two stored memory blocks through a library call when its pointer is nonzero.
- `scan_string_between_library_calls`: Calls a library function, measures a zero-terminated string from an address held in a register, then calls a second library function.
- `unpack_bitstream_to_buffer`: Reads a bitstream from a source buffer, using a checksum for validation, and expands the packed bits into a destination buffer by writing bytes derived from the bit sequence.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `output_bits_to_color_register`: Writes successive bits from a packed value to the background colour register, refilling the value from the input when it runs out, for the requested number of steps.

### Bionic Commando.adf:DEFJAM_3

- `call_relocation_handler`: Calls a routine to process memory hunk relocations and then jumps to address zero.
- `process_amiga_hunk_relocations`: Parses a list of memory hunks by comparing hunk type constants and performs relocation updates.
- `update_pointer_table`: Reads a count and a series of index pairs from a source list, then uses those indices to fetch values from a table and add them to a destination pointer in a loop.
- `apply_bitwise_or_constants`: Performs several bitwise OR operations on registers d0 and d1 using specific constants.
- `unpack_stream_with_pointer_shift`: Decompresses a packed stream by reading bits through a shift routine that takes a 32-bit value from the four bytes before the source pointer, shifts it right and sets its top bit, and writes the resulting bytes to a destination buffer.
- `shift_pointer_value`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `shift_bitstream_store_pointer`: Repeats a shift of the bit buffer right by one place for a count in d1, storing the current pointer's low word into a global on each pass.

### Bionic Commando.adf:DEFJAM_4

- `find_max_and_prepare_buffer_2`: Walks a linked list to find the maximum value in a field, then writes a header and a sequence of words into a buffer.
- `move_register_to_d0`: Moves the contents of register d1 into register d0 and branches to another routine.

### Bionic Commando.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_sequential_subroutines`: Saves registers, calls four subroutines in sequence, clears register d0, and returns.
- `swap_graphics_field_and_poll_status`: Opens the graphics library, swaps a value in a structure field with a new value, and loops calling three routines while a bit in the CIAA port remains set.
- `copy_longwords_until_sentinel`: Copies 32-bit words one by one from a source table into a destination buffer.
- `clear_memory_range`: Clears a range of memory starting at a specific address up to a limit, then stores the value sixteen in a global variable.
- `scan_text_rows_until_terminator`: Walks a text table in rows of 37 bytes, calling another routine for each nonzero byte.
- `copy_data_block`: Calculates an offset based on the input byte, then copies eight 32-byte blocks from a source table to a destination table using a loop, and finally increments a global variable.
- `modify_bit_in_memory`: Reads a byte from the keyboard serial register, performs arithmetic and bitwise operations on it, and if the result is between 0 and 7, clears the register and toggles a bit in a global variable at a position determined by the processed value.
- `call_string_formatters`: Clears register d0 and calls another routine nine times, passing a different address from a list of nine constants in a0 each time.
- `select_string_by_bit_test`: Tests a bit in a longword from a global variable using a bit index in register d0; if the bit is zero, it writes the string ' ON ' to the location pointed to by a0, otherwise it writes ' SEY ' to that location, then increments d0 and returns.
- `call_library_functions`: Calls three different functions from a library retrieved from a pointer at a fixed offset, using the return value of the first call as the base for the subsequent two calls.
