# Game 1232 (mean arenas (1993)(ice)[cr dlm](disk 1 of 2)[t +22 god]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01232). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904112 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 12 raw-word lines |
| Functions named and described | pass, 68 of 68 named (0 author symbols), 68 of 68 described (5 by rule, 63 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 683 of 683 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 6656 bytes, 37 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1977 of 2019 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 3741 |
| code | 7156 |
| data | 892702 |
| padding | 1 |
| unused | 512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Mean Arenas (1993)(ICE)[cr DLM](Disk 1 of 2)[t +22 GOD].adf:bootblock | 1 | 278 | 4 |
| logged disk load from Mean Arenas (1993)(ICE)[cr DLM](Disk 1 of 2)[t +22 GOD].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 768 | 27 |
| Mean Arenas (1993)(ICE)[cr DLM](Disk 1 of 2)[t +22 GOD].adf bytes 1024-7680 (code never loaded by a reference run) | 1 | 973 | 37 |

## Cases handled

- mean arenas (1993)(ice)[cr dlm](disk 1 of 2)[t +22 god]/Mean Arenas (1993)(ICE)[cr DLM](Disk 1 of 2)[t +22 GOD].adf: the boot block reads 4096 bytes from disk offset 12800 to an address (found statically); recovered as a unit
- runtime segment an address: 521 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- mean arenas (1993)(ice)[cr dlm](disk 1 of 2)[t +22 god]/Mean Arenas (1993)(ICE)[cr DLM](Disk 1 of 2)[t +22 GOD].adf: bytes 1024-7680 look like code no run loaded: recovered position-independently (load address unknown)
- mean arenas (1993)(ice)[cr dlm](disk 1 of 2)[t +22 god]/Mean Arenas (1993)(ICE)[cr DLM](Disk 1 of 2)[t +22 GOD].adf: 2 logged loads cover 10752 bytes of the image
- mean arenas (1993)(ice)[cr dlm](disk 1 of 2)[t +22 god]/Mean Arenas (1993)(ICE)[cr DLM](Disk 1 of 2)[t +22 GOD].adf: blocks 16-24 (bytes 8192-12800) outside the filesystem hold data, not code (opcode density 0.022, entropy 3.21)
- mean arenas (1993)(ice)[cr dlm](disk 1 of 2)[t +22 god]/Mean Arenas (1993)(ICE)[cr DLM](Disk 1 of 2)[t +22 GOD].adf: blocks 33-879 (bytes 16896-450560) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.63)
- mean arenas (1993)(ice)[cr dlm](disk 1 of 2)[t +22 god]/Mean Arenas (1993)(ICE)[cr DLM](Disk 1 of 2)[t +22 GOD].adf: blocks 882-1759 (bytes 451584-901120) outside the filesystem hold data, not code (opcode density 0.022, entropy 6.58)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `setup_io_request_two` (Mean Arenas (1993)(ICE)[cr DLM](Disk 1 of 2)[t +22 GOD].adf bytes 1024-7680 (code never loaded by a reference run)): Sets four consecutive memory locations with specific values and a value from a pointer, then executes a function via a pointer stored in a global variable. Kind: disk/track check. Pass path: the branch following the call (the code skips the BSR if the check passes, or the BSR is the check itself and the subsequent instruction is the pass path).
- `prepare_io_request_parameters` (Mean Arenas (1993)(ICE)[cr DLM](Disk 1 of 2)[t +22 GOD].adf bytes 1024-7680 (code never loaded by a reference run)): Sets four consecutive memory locations with the values three, four thousand five hundred ninety-two, a value from a pointer at offset eight of a structure in a5, and seven thousand one hundred sixty-eight, then calls a function via a pointer stored in a global variable. Kind: track/disk integrity check. Pass path: the branch following the tst.l an address(a5) instruction (likely when the result is zero/non-error).
- `call_do_i_o` (Mean Arenas (1993)(ICE)[cr DLM](Disk 1 of 2)[t +22 GOD].adf bytes 1024-7680 (code never loaded by a reference run)): Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller. Kind: disk/track check. Pass path: the branch not taken by the conditional jump (e.g., if tst.w an address.l results in zero, the code continues past an address).
- `execute_io_request_e` (Mean Arenas (1993)(ICE)[cr DLM](Disk 1 of 2)[t +22 GOD].adf bytes 1024-7680 (code never loaded by a reference run)): Clears a memory location, sets a value of 14 at an offset in a structure, and calls a function via a pointer in a global variable. Kind: disk/track integrity check. Pass path: the branch taken when the bne.b/bne.w condition is NOT met (the fall-through path).
- `execute_io_request_d` (Mean Arenas (1993)(ICE)[cr DLM](Disk 1 of 2)[t +22 GOD].adf bytes 1024-7680 (code never loaded by a reference run)): Clears a memory location, sets a value of 13 at an offset in a structure, and calls a function via a pointer in a global variable. Kind: track/disk integrity check. Pass path: beq.b an address.
- `execute_io_request_f` (Mean Arenas (1993)(ICE)[cr DLM](Disk 1 of 2)[t +22 GOD].adf bytes 1024-7680 (code never loaded by a reference run)): Clears a memory location, sets a value of 15 at an offset in a structure, and calls a function via a pointer in a global variable. Kind: disk/track check. Pass path: bne.w an address (or similar bne/beq branches depending on the specific call site's logic flow).
- `setup_io_with_scaled_input` (Mean Arenas (1993)(ICE)[cr DLM](Disk 1 of 2)[t +22 GOD].adf bytes 1024-7680 (code never loaded by a reference run)): Populates a table with a global variable and a calculated value based on an input, then calls an I/O routine. Kind: disk/track integrity check. Pass path: execution continues to the next instruction (the routine returns normally to the caller).
- `setup_io_request_three_scaled` (Mean Arenas (1993)(ICE)[cr DLM](Disk 1 of 2)[t +22 GOD].adf bytes 1024-7680 (code never loaded by a reference run)): Populates a table with a global variable and a calculated value based on an input, then calls an I/O routine using a different constant. Kind: track/disk integrity check. Pass path: the branch following the call to the function pointer (the code continues execution normally after the indirect call returns).
- `execute_io_request_scaled_b` (Mean Arenas (1993)(ICE)[cr DLM](Disk 1 of 2)[t +22 GOD].adf bytes 1024-7680 (code never loaded by a reference run)): Sets a value of 11 in a structure, copies a pointer from a register to a structure, calculates a value by multiplying a register by 5632, and jumps to a function via a pointer in a global variable. Kind: disk/track integrity check. Pass path: the branch following the 'tst.l d0' instruction where d0 is non-zero (success).
- `call_do_i_o_2` (Mean Arenas (1993)(ICE)[cr DLM](Disk 1 of 2)[t +22 GOD].adf bytes 1024-7680 (code never loaded by a reference run)): Wrapper that calls the exec library function DoIO and returns its result to the caller. Kind: disk/track integrity check. Pass path: the branch taken when an address(a5) is non-zero (the 'bne' instruction).

## Functions


### Mean Arenas (1993)(ICE)[cr DLM](Disk 1 of 2)[t +22 GOD].adf:bootblock

- `initialize_boot_sequence_2`: Sets up a structure with specific pointers and values, allocates memory, opens a device, performs several I/O operations, and executes a task before potentially setting a global longword.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `initialize_boot_sequence`: Sets four values in a structure, calls a function via a pointer in a global variable, calls a routine at address an address, and writes a constant to a memory location before branching.
- `decompress_and_expand_data`: Checks for a specific magic signature at the start of a buffer, then performs a multi-stage decompression and expansion process involving bit-shifting and table lookups to populate a destination memory area.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_graphics_and_hardware`: Allocates memory, initializes a structure with hardware register values and bitwise flags, and configures DMA and interrupt settings.
- `store_long_value`: Stores a 32-bit constant into a location at offset 882 of a structure pointed to by A0 and returns zero in D0.
- `compare_and_call_pointer`: Adds a constant to a pointer in a1, compares the word at that location to 17920, and if equal, branches to another location; otherwise, it saves a0 to the stack and returns.
- `call_pointer_from_memory`: Loads a longword from a memory location into a0, calls the routine at that address, and then branches to another location.
- `initialize_structure_data`: Copies a pointer from A1 to a global variable, writes the byte 74 to four specific offsets in the structure in A1, and writes several 16-bit values to various offsets in the structure in A1 or a modified pointer in A2.
- `apply_bitwise_or_sequence`: Performs a series of bitwise OR operations on the byte in D4 using several different immediate values.
- `initialize_memory_values`: Writes several words and a byte to specific offsets in memory using a2 and a1, then loads the address of the next instruction into a2.
- `tiny_movea_rts`: Small routine of 2 instructions that copies (a7)+ to a2, then returns.
- `branch_on_byte_value`: Tests a byte at a specific memory location; if it is zero, it branches to another routine, otherwise it loads the byte into d0, clears the memory location, and jumps to a calculated address in a1 based on whether d0 equals 249.
- `jump_to_offset_address`: Adds a constant to the pointer in A1 and jumps to the resulting address.
- `write_global_word_and_return_registers`: The routine stores a word to a global variable and returns the concatenation of two registers.
- `clear_array_to_zero`: The routine clears a block of 32-bit words to zero.
- `prepare_word_for_call`: Loads the word value 59775 into d0, calls another routine, and then increments d0 by 1.
- `wait_for_word_match`: The routine loops until the low word of a global variable, masked to nine bits, equals a word value in D0.
- `iterate_table_and_advance_pointer`: The routine walks a byte table terminated by -1, calling another routine for each entry and updating a pointer from an output value.
- `copy_sprite_pattern_bytes`: The routine copies a series of bytes from a table pointed to by a global variable offset by a byte in D0, using a stride taken from another global variable, into a destination buffer pointed to by A1 at fixed offsets.
- `count_string_bytes_and_store_half`: The routine counts the number of bytes in a zero-terminated string pointed to by a pointer, stopping at the byte with value -1, and stores the negated half-length minus a constant into a 16-bit field at an offset from an address in A5.
- `check_byte_range`: Loads a byte from a table using an offset in a6, and returns if the byte is between 248 and 249 or between 8 and 11; otherwise, it branches to another location.
- `verify_byte_range`: Loads a word from offset 16 of a structure, and if it is not zero, checks if the value minus one is a valid index into a table pointed to by offset 566 of the structure.
- `search_table_for_non_zero_byte`: Compares a value from a structure at offset 16 of a5 with a byte from a table; if they are not equal, it iterates through the table until it finds a non-zero byte, then updates several global values and a structure offset.
- `decrement_counter_and_wrap_value`: The routine decrements a byte at an offset from a pointer in A5 and, if it reaches zero, resets it to one and updates a short counter at another offset, clamping it between zero and fifteen.
- `decode_command_stream`: Reads a byte stream from a pointer in a5, converts each byte by subtracting 48, writes the result to a buffer in a3, and depending on the value, writes a following byte to a buffer in a4 or calls another routine.
- `process_table_entry_and_update_pointers`: Decrements a byte at offset 358 of a structure and, if non-zero, checks hardware status bits to decide whether to clear the value or proceed.
- `process_table_entries_and_format_strings`: Iterates through a byte table, updating a pointer and a counter for each entry.
- `parse_string_table`: Walks a global string table and copies each character into a global buffer, subtracting 48 from each byte, and sets a 16-bit value at a global location to 1 if a character is the digit 2, otherwise it sets it to 0.
- `decrement_numeric_values`: Decrements a 16-bit counter in a global variable and, if not zero, iterates through a list of pointers to decrement specific nibbles of the 16-bit values they point to, returning 1 if any value was changed.
- `increment_numeric_values`: Increments a 16-bit counter in a global variable and, if not zero, iterates through a list of pointers to increment specific nibbles of the 16-bit values they point to, returning 1 if any value was changed.

### Mean Arenas (1993)(ICE)[cr DLM](Disk 1 of 2)[t +22 GOD].adf bytes 1024-7680 (code never loaded by a reference run)

- `setup_hardware_and_memory`: Performs several bitwise operations on d0, calls another routine, and writes a value to the custom chip color registers.
- `dispatch_io_requests`: Initializes several memory locations and calls a series of subroutines to handle different I/O request types, returning different error codes in d0 based on the execution path.
- `run_decrement_loop`: Decrements a longword from an address down to 0 using a loop and returns.
- `branch_to_subroutine`: Saves registers to the stack, loads a pointer from an offset of the program counter into a6, and branches to another routine.
- `check_and_branch`: Saves registers to the stack, loads a pointer from an offset of the program counter into a5, tests a longword at offset 24 of that pointer, and either calls another routine or branches to another routine.
- `initialize_system_subsystems`: Clears a value at an offset of a structure, calls another routine, and if a specific offset in that structure is zero, it calls two more routines before setting two global longwords and returning.
- `manage_memory_hunks`: Updates a series of memory pointers by iterating through a list of relocation offsets and copying data blocks of varying sizes.
- `execute_io_sequence_2`: Performs a complex sequence of I/O operations by calling multiple routines, managing several global pointers, and performing arithmetic to update memory buffers and structure offsets.
- `execute_io_sequence`: Clears a value in a structure, performs several conditional calls to other routines, and manages a pointer in a structure while potentially performing I/O operations and updating a value in a structure via a multiplication.
- `initialize_data_structure`: Clears a value at offset 36 of a structure pointed to by a global variable, stores a longword from d1 into another global variable, and calls another routine.
- `write_string_data`: Copies a specific string into a memory location and sets several bytes at an offset from that location.
- `populate_data_structure`: Writes a large sequence of multi-byte values and strings into a memory block pointed to by a value in a register.
- `write_short_string`: Writes a small sequence of bytes and a longword into a memory block pointed to by a value in a register.
- `copy_memory_block`: Copies a sequence of bytes from one memory location to another using a counter.
- `decompress_compressed_data_stream`: Decompresses a data stream into a destination buffer using a custom algorithm that processes variable-length bitfields, length descriptors, and a lookup table to expand compressed bytes.
- `relocate_hunk_data`: Checks a structure for specific AmigaDOS hunk type codes and, if they match, iterates through a list of relocation offsets to update pointers in a memory block, then stores the new block address in a global variable.
- `compare_case_insensitive`: Performs a case-insensitive comparison between two null-terminated strings by checking characters and adjusting for case differences.
- `get_string_identifier`: Checks if a four-byte signature exists at a pointer offset and returns a value from a subsequent offset if the signature matches.
- `validate_string_list`: Iterates through a list of entries to verify that a global string matches a target string using case-insensitive comparison.
- `check_table_termination`: Iterates through a table of entries until it finds a pair of non-zero values at a specific offset.
- `find_first_set_bit`: Scans a memory buffer byte by byte to find the index of the first bit set to one.
- `count_zero_bits`: Scans a block of memory and counts the total number of zero bits present in the data.
- `clear_bit_in_buffer`: Clears a specific bit within a buffer at a calculated offset based on the input value.
- `set_bit_in_buffer`: Sets a specific bit within a buffer at a calculated offset based on the input value.
- `copy_filtered_string`: Copies a sequence of characters from one buffer to another, skipping spaces, tabs, or newlines, and terminates the destination with null bytes.
- `manage_memory_and_io`: Checks a global pointer, allocates memory of size 512 if null, then checks a structure offset 8 and allocates 5120 bytes if null, finally calls a routine via a pointer in the structure and another routine via a pointer in the global variable.
- `call_close_device`: Wrapper that calls the exec library function CloseDevice and returns its result to the caller.
- `setup_io_request_two`: Sets four consecutive memory locations with specific values and a value from a pointer, then executes a function via a pointer stored in a global variable.
- `prepare_io_request_parameters`: Sets four consecutive memory locations with the values three, four thousand five hundred ninety-two, a value from a pointer at offset eight of a structure in a5, and seven thousand one hundred sixty-eight, then calls a function via a pointer stored in a global variable.
- `call_do_i_o`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `execute_io_request_e`: Clears a memory location, sets a value of 14 at an offset in a structure, and calls a function via a pointer in a global variable.
- `execute_io_request_d`: Clears a memory location, sets a value of 13 at an offset in a structure, and calls a function via a pointer in a global variable.
- `execute_io_request_f`: Clears a memory location, sets a value of 15 at an offset in a structure, and calls a function via a pointer in a global variable.
- `setup_io_with_scaled_input`: Populates a table with a global variable and a calculated value based on an input, then calls an I/O routine.
- `setup_io_request_three_scaled`: Populates a table with a global variable and a calculated value based on an input, then calls an I/O routine using a different constant.
- `execute_io_request_scaled_b`: Sets a value of 11 in a structure, copies a pointer from a register to a structure, calculates a value by multiplying a register by 5632, and jumps to a function via a pointer in a global variable.
- `call_do_i_o_2`: Wrapper that calls the exec library function DoIO and returns its result to the caller.
