# Game 2454 (powerdrome (1989)(electronic arts)[cr black monks][t +4 black monks]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02454). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (942624 top-level bytes in 54 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 42 raw-word lines |
| Functions named and described | pass, 358 of 358 named (0 author symbols), 358 of 358 described (24 by rule, 325 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3312 of 3312 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 99668 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8389 of 8442 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 386536 |
| code | 33222 |
| data | 434222 |
| padding | 12868 |
| unused | 75776 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Powerdrome (1989)(Electronic Arts)[cr Black Monks][t +4 Black Monks].adf:pd | 3 | 228 | 7 |
| Port-Handler | 1 | 306 | 1 |
| Disk-Validator | 1 | 386 | 1 |
| serial.device | 4 | 976 | 83 |
| Sounds2 | 1 | 1 | 1 |
| Sounds3 | 1 | 91 | 1 |
| Sounds | 1 | 10 | 1 |
| translator.library | 2 | 490 | 50 |
| version.library | 2 | 41 | 7 |
| Powerdrome (1989)(Electronic Arts)[cr Black Monks][t +4 Black Monks].adf:bootblock | 1 | 197 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2859 | 94 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2642 | 99 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 211 | 6 |
| Powerdrome (1989)(Electronic Arts)[cr Black Monks][t +4 Black Monks].adf:pd.bin (raw code file, load address unknown) | 1 | 3 | 1 |

## Cases handled

- powerdrome (1989)(electronic arts)[cr black monks][t +4 black monks]/Powerdrome (1989)(Electronic Arts)[cr Black Monks][t +4 Black Monks].adf:PDSounds/Cheer: high-entropy data file (7.78 bits/byte), classified as packed data
- powerdrome (1989)(electronic arts)[cr black monks][t +4 black monks]/Powerdrome (1989)(Electronic Arts)[cr Black Monks][t +4 Black Monks].adf:PDSounds/Thunder: high-entropy data file (7.69 bits/byte), classified as packed data
- powerdrome (1989)(electronic arts)[cr black monks][t +4 black monks]/Powerdrome (1989)(Electronic Arts)[cr Black Monks][t +4 Black Monks].adf:PDData/Demo.Lap: high-entropy data file (7.67 bits/byte), classified as packed data
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1896 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1008 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 209 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- powerdrome (1989)(electronic arts)[cr black monks][t +4 black monks]/Powerdrome (1989)(Electronic Arts)[cr Black Monks][t +4 Black Monks].adf:pd.bin: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 99668 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Powerdrome (1989)(Electronic Arts)[cr Black Monks][t +4 Black Monks].adf:pd

- `branch_then_jump_entry`: Branches to the decompression and relocation routine and then unconditionally jumps to address zero, which is the start of the relocated code.
- `decompress_and_relocate_table`: Decompresses a bitstream into a table of 32-bit values, then relocates the entries in that table by adding offsets from a second table and rewriting them as pointers, before returning.
- `add_bytes_via_pointers`: Loads the byte at the address in a0 into a0, adds the byte at the address in a1 to it, stores the result back through a0, then returns.
- `or_bits_in_d0`: It applies a series of bitwise OR operations to the 8-bit value in d0 using the constants 92 and 220, then performs three no-op ORs with zero and returns.
- `unpack_stream_with_pointer_shift`: It loads a 32-bit value from the four bytes before the source pointer, shifts it right by one bit, and sets its top bit, refilling the bit buffer when it empties.
- `shift_pointer_value`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `shift_and_rotate_d0`: It decrements a loop counter in d1 and copies a0 into the color register at a fixed address, then shifts d0 right and loads the next word from a0 when the shifted value becomes zero.

### Port-Handler

- `initialize_port_handler`: Initializes a structure at a1 by storing values read from the table at a0, sets up several pointer fields, zeroes a block of 31 words, and calls a routine; if a flag at offset 316 of a1 is nonzero it calls the routine again and jumps into a table, otherwise it copies 31 words from one table to another and patches several entries.

### Disk-Validator

- `store_digit_and_scan_nodes`: Writes a digit character into a two-byte field of a table indexed by a counter in a global structure, then decrements that counter and copies a 32-bit value from one field to another.

### serial.device

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `reinit_serial_buffer_fields`: If the value in a1 is not zero, it jumps through a function pointer stored at offset 62 of the structure in a6 plus a negative offset, then stores a value from d2 into several fields of that structure and writes shifted copies of it at higher offsets.
- `check_flag_and_call_handler`: The routine tests a byte at an offset from a pointer in A1 against a mask and, if the test passes, calls a handler routine obtained from a table via A6.
- `set_status_and_call_handler`: Writes the byte 253 to the status byte at offset 31 of the structure in a1, then, if a masked test of the byte at offset 30 passes, it calls a handler routine obtained from a table in a6.
- `enable_interrupts_call`: Sets the interrupt enable register to enable interrupts, calls a routine, then sets the register again before returning.
- `handle_serial_request_status`: Clears a byte at offset 1f of the structure in a1, then compares a word at offset 1c against the constants two, three, and ten, branching to different handlers for each match before falling through to a default branch.
- `check_and_clear_pointer`: Compares a pointer in a1 with the one at offset 74 of the structure in a6, and if they differ it clears bit 1 of the status byte at offset 472, clears the pointer at offset 74, and jumps to the shared exit.
- `unlink_second_queue_request_2`: Checks bit 6 of a status byte, and if set it unlinks that structure from a doubly linked list by relinking its neighbours, then decrements a counter and branches onward.
- `handle_serial_library_call`: Compares a pointer against a stored value and checks a field against a constant, then clears a flag, writes to a hardware register, calls a routine, clears the pointer and clears another flag before branching onward.
- `unlink_second_queue_request`: If bit 6 of the status byte at offset 30 of the structure in a1 is set, it unlinks that structure from a doubly linked list, decrements a counter in a6, flags the next entry, and calls a handler routine via a pointer in a6 when a low bit is clear.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `check_status_then_set_flag`: Clears the return value in d0, then checks a 16-bit status field at offset 32 of the structure in a6 and returns immediately if it is zero, otherwise sets bit 6 of the byte at offset 296 of that structure and returns.
- `invoke_library_vectors_then_unlink`: Reads a longword from offset 438 of the structure in a6 and calls the routine at that address twice, invokes several other routines through pointers in a table, saves a value from offset 426, then unlinks the head of a doubly linked list and returns the saved value.
- `insert_request_into_serial_list`: Clears bit 8 of the status byte at offset 30, and if the request is pending and a comparison passes it clears bit 7, then inserts the structure into a list in a6, increments a counter, and calls a handler routine; otherwise it calls the handler when a masked test passes.
- `process_serial_device_queue`: Pops the front entry from a queue stored in the structure in a6, updating the head and tail pointers and marking the removed entry, then processes buffered serial characters by advancing read and write pointers and updating status flags, otherwise it returns.
- `process_serial_buffer`: Processes the next character from a serial buffer in the structure in a6, adjusting the read and write pointers and decrementing the remaining count, then clears the corresponding receive bit in a global variable or, if that bit is not set, clears the transmit bit before storing the character through a pointer and advancing a counter.
- `test_status_bit_branch`: Tests bit 0 of the byte at offset 189 of the structure in a6 and, if that bit is clear, branches to a routine at offset 194 that processes a queue and may store the value 9 at offset 31 of the structure in a1, otherwise it returns.
- `process_queue_then_flag`: Calls a queue-processing routine, and if it returns zero it stores the value 9 in the byte at offset 31 of the structure in a1 before jumping to the shared exit.
- `process_device_queue`: Clears a byte at offset 74 of the structure in a6, increments a counter at offset 72 and a byte at offset 82, then processes a queue by removing an entry, setting its flags, and calling another routine when the queue is non-empty.
- `restore_registers_and_return`: Restores the registers d2 and a6 from the stack and returns immediately.
- `identify_routine`: This routine is not empty; it contains ten instructions but the decompiler has not produced meaningful pseudo-code.
- `clear_status_bit_store_2`: Clears a status bit and, if that bit had been set, stores a value in another structure before jumping away.
- `clear_status_bit_store`: Clears bit 7 of the byte at offset 1d6 of the structure in a6 and, if that bit had been set, stores the value 9 in the byte at offset 31 of the structure in a1 before branching to a routine at offset 1d4, otherwise it returns.
- `configure_device_status`: Stores the value 12 in the byte at offset 31 of the structure in a1, then tests bit 0 of the status byte at offset 470 and stores the value 6 there only when that bit is set.
- `process_serial_data_buffer`: Initializes a global counter to 2048, then processes serial status flags and data in the structure in a6, writing a byte to a buffer pointer and advancing the buffer indices when space is available, and finally calls the routine at the address stored at offset 62 of the structure.
- `check_and_enable_serial_interrupts`: Compares a byte from a serial status register in a6 with a value in d1 and returns immediately if they differ, otherwise clears a bit in a serial control register in a6 and enables interrupts.
- `test_serial_status_bits`: Tests bit 5 of the byte at offset 29 of the structure in a6 and returns immediately if it is clear, then tests bit 1 of the byte at offset 189 of the same structure and returns if it is set, otherwise it calls another routine and returns if that call's result is zero, then tests bit 8 of the value in d0 and returns if it is clear.
- `test_bit_branch`: Tests a bit in a register and branches based on whether it is set or clear.
- `test_bit_forward_control`: Tests bit 0 of the byte at offset 29 of the structure in a6 and always jumps to the routine at 2370, so it only forwards control and returns there.
- `set_status_bit_test`: Sets bit 7 of the byte at offset 1d6 of the structure in a6, then tests the longword at offset 62 and, if it is nonzero, branches to a routine at offset 1d4, otherwise it returns.
- `manage_buffer_counters`: Reads the longword at offset 54 and compares it with the one at offset 58, and if the first is smaller it stores a byte, increments the counters at offsets 58 and 42, updates the pointer at offset 42 when the one at offset 38 is exceeded, and branches back to a loop when the counter is within range, otherwise it stores the pointer at offset 62 when the one at offset 42 is zero.
- `record_pointer_then_signal`: Tests a longword at offset 62 of the structure in a6, and if it is zero it copies the longword from offset 42 to offset 62.
- `update_parallel_port`: Copies the byte at offset 34 of the structure in a6 to the port register at offset 473, clears bit 3 of the status byte at offset 472, and if bit 2 of the byte at offset 69 is set it clears bit 6 of the parallel port register.
- `copy_and_configure_interrupts`: Copies the byte at offset 35 of the structure in a6 to offset 29 of the same structure, sets bit 3 of the byte at offset 28, and if bit 2 of the byte at offset 69 is clear sets bit 6 of the interrupt enable register, otherwise sets bit 15 of the interrupt mask and returns.
- `clear_serial_state_byte`: Clears the byte at offset 473 of the structure in a6 and returns.
- `set_flag_and_call`: Tests bit 6 of the byte at offset 1d6 and, if it is clear, sets that bit, zeroes the long at offset 1d2, copies the longword from offset 6a, decrements the integer at offset 1d2, stores the value 9 at offset 19e, calls a routine obtained from the pointer at offset 196, and stores the value 11 at offset 1f of the structure in a1 when the byte there is nonzero, otherwise it returns.
- `read_serial_data_byte`: Saves registers d2, a3 and a6 on the stack, copies a1 into a6, then waits for the serial data register to hold a byte before setting its status bits and jumping to the handler.
- `load_offset_call_routine`: Loads the address of the value at offset 24 of the structure in a6 into a0, calls the routine at 1512, then jumps to the routine at 2276.
- `signal_serial_request`: Sets bit 0 of the status byte at offset 1d6 of the structure in a6, then jumps to the handler routine at 962.
- `manage_request_queue`: Clears the long at offset 32 of the structure in a1, and if the long at offset 36 is nonzero it clears bit 1 of the flag byte at offset 30.
- `check_flags_update_state`: Tests bit 2 of the byte at offset 69 of the structure in a6 and if it is set calls the routine at 2868, then tests bit 7 of the byte at offset 31 of the structure in a1 and if it is set clears a 32-bit value at offset 94, copies a pointer from offset 40 of a1 to offset 86, copies the pointer in a1 to offset 78, and enables interrupts via the two mask registers before returning.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_request_state_on_flags`: Reads the parallel port register and, if specific bits are set, writes the byte 13 to the status byte at offset 31 of the structure in a1; otherwise it leaves the memory unchanged.
- `remove_head_serial_request`: Removes the head of a doubly linked list in the structure in a1 by relinking its neighbours and decrementing the counter at offset 72, then sets and clears bits in the status byte at offset 30 of the removed node.
- `store_serial_request_byte`: Stores the byte value 200 into the byte at offset 220 of the structure in a6, loads a pointer from offset 86 of that structure into a0, clears the low word of d0, reads a byte from the address in a0 into d0, and loads the longword at offset 36 of the structure in a1 into d1, then returns if the sign bit of d1 is clear.
- `handle_serial_request`: Processes a serial request by updating status flags, computing a response value, scanning a byte buffer, and copying a longword from a structure into a global variable before returning.
- `process_serial_request`: Sets a bit in a status byte based on a value from another routine, ORs it with a stored word, saves the result to a global variable, increments two counters, scans a table for the lowest matching entry, and if a counter exceeds a stored value it stores the counter and calls a routine.
- `remove_head_request`: If the counter at offset 72 is nonzero it removes the head of the list, relinks its neighbours, decrements the counter, and sets and clears bits of the flag byte at offset 30 of the removed structure.
- `restore_registers_and_enable_interrupts`: Restores d2 and a6 from the stack, sets a global variable to one, enables interrupts, and returns.
- `combine_and_check_request`: Sets bit 15 of the DMA enable register, combines the byte at offset 29 of the structure in a1 with the word at offset 24 into d1, clears that byte, writes the combined value to the serial data register, and if the long at offset 78 is nonzero it tests the word at offset 72 and returns, otherwise it falls through.
- `initialize_serial_device`: Initializes the serial device by writing configuration values into a structure, clearing status flags, and calling setup routines before enabling interrupts.
- `call_then_check_and_call_handler`: Calls a routine via a branch, then, if a masked test of the status byte at offset 30 of the structure in a1 passes, it invokes a handler routine obtained from a table in a6.
- `copy_serial_state`: The routine copies a 32-bit value from an offset within a structure pointed to by a register to two other offsets in the same structure, then clears a 32-bit field at a third offset.
- `handle_pending_requests`: Tests the pointer at offset 78 and sets bit 2 of the status byte at offset 472 when it is present, then tests the pointer at offset 74 and calls another routine when it is present.
- `clear_bit_call_handler`: Clears bit 3 of the byte at offset 1d8, then if the longword at offset 74 is nonzero it calls a routine and, when bit 0 or bit 6 of the byte at offset 30 of the structure in a1 is clear, calls a routine obtained from the pointer at offset 62, otherwise it returns zero.
- `enable_interrupts_and_check_flag`: Enables interrupts by writing to the interrupt enable register, then calls a routine that tests a byte at offset 30 of the structure in a1 against a mask and invokes a handler pointer stored at offset 62 of a6 when the test passes.
- `clear_serial_flag`: The routine calls another routine twice and then clears a 32-bit field in a structure pointed to by an address register.
- `load_and_store_pointer_value`: Copies a pointer from a global variable into a1, loads the value it points to into d0, and returns immediately if that value is zero.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `set_serial_flags_and_maybe_call`: The routine sets a flag byte in a structure pointed to by a1 and writes a constant to the next byte.
- `store_serial_status_byte`: Builds a status byte from a global flag and two bits in the structure in a6, stores it at offset 81 of the structure in a1, copies a byte to offset 80 and masks it, copies a long at offset 32, clears a bit at offset 470, then calls the handler routine if the byte at offset 30 of the structure in a1 matches the mask.
- `clear_flag_test_queue`: Clears bit 0 of the byte at offset 30 of the structure in a1, then tests bit 3 of the byte at offset 79 and, if it is set, tests the word at offset 72 and the longword at offset 78, branching to a routine at offset 2ad2 when either is nonzero, otherwise it returns.
- `load_and_start_request`: Loads the long at offset 106 of the structure in a6 into d0, calls the routine at 3866, then tests bit 7 of the byte at offset 31 of a1 and if it is set copies the pointer in a1 to offset 78, sets bit 5 of the byte at offset 29, writes the word 34816 to the serial control register, and jumps to the routine at 3794.
- `reply_and_clear_active_request`: Pushes a6 onto the stack, updates a6 using a pointer at offset 98, calls a routine via a pointer at offset 98, restores a6 from the stack, clears a 32-bit value at offset 78 of the structure in a6, and returns.
- `clear_uart_break_and_reply`: Clears bit 5 of a byte at offset 470 of the structure in a1, and if bit 5 of that byte is set, a non-zero value is in a6, and a short at offset 78 of a pointer in a1 is 10, it sets bit 1 of the byte, clears a pointer in a1, writes 2048 to a global variable, calls a routine via a pointer, and sets a global variable to 32769.
- `start_timed_read_request`: Sets a global variable to one, clears a 32-bit value at offset 142 of a structure in a6, stores a 32-bit value from d0 at offset 146, sets a 16-bit value at offset 138 to nine, and calls a routine via a pointer.
- `check_parallel_status_bits`: Tests bit 7 of the byte at offset 79 of the structure in a1 and returns if it is clear, then sets bit 7 of the byte at offset 69 of the structure in a6 and clears bit 2 of the byte at offset 184 of the same structure, then tests bit 3 of that byte and returns if it is clear, otherwise it clears bit 7 of the byte at offset 79 of a1, calls another routine, sets bit 7 of that byte, and returns.
- `check_serial_request_pending`: Clears a status bit, then checks several flags and returns early unless a request is pending, in which case it sets a byte at offset 1f and jumps to the handler.
- `enable_disable_interrupts`: Writes the value 2049 to the interrupt enable register to enable interrupts, calls a routine that processes a queue and may call a handler, writes the value 34817 to the same register to disable interrupts, and, when bit 0 or bit 6 of the byte at offset 30 of the structure in a1 is clear, calls a routine obtained from the pointer at offset 62, otherwise it returns.
- `copy_serial_device_config`: Copies the serial device's channel configuration and control bytes from the structure in a1 into the corresponding fields of the structure in a6, adjusts several control bits based on flags in the source, then calls another routine to process the copied data.
- `compute_unit_channel_masks`: Calculates a 16-bit mask based on a byte at offset 77 of a1, modifies it based on a byte at offset 78 of a1 and bit 0 of a byte at offset 69 of a6, stores the result at offset 468 of the structure in a6, and calls two other routines to store results at offsets 466 and 464.
- `decrement_indexed_table_lookup`: The routine checks a flag byte in a structure pointed to by a register and conditionally decrements a short index.
- `store_byte_forward_control`: Stores the byte value 5 at offset 31 of the structure in a1 and always jumps to the routine at 4318, so it only sets that byte and forwards control.
- `scale_and_divide_value`: Loads the long at offset 258 of the structure in a6 into d1, doubles it into d0 by shifting, subtracts the original, compares the result with 65535 and if it is greater shifts it right by five bits, divides d1 by that value, masks the quotient to 16 bits, shifts it right by five bits, and jumps to the routine at 4472.
- `store_serial_period_quotient`: Divides a 32-bit value in d1 by a 16-bit value in d0, stores the quotient in a structure in a6 and a global variable, clears bit 5 of a byte at offset 472 of a6, and if a byte at offset 66 of a6 is 8 and bit 0 of a byte at offset 69 of a6 is set, it performs a bitwise OR on the quotient and sets bit 5 of the byte at offset 472.
- `call_library_open_device`: Reads a pointer from offset 84 of the base in a6 and calls the routine at that address, and if the result is nonzero it saves the base pointer at offset 98, stores the decimal value 200 at offset 476, and calls a second library routine that returns a status flag which is stored at offset 438.
- `setup_serial_request_struct`: Stores the decimal value 5 at offset 362 of the structure in a6 and writes the pointer at offset 394 into the 32-bit field at offset 304, then sets the handler address at offset 136 and stores the result of a library call at offset 430 before writing another handler address at offset 158 and storing the second library result at offset 434.
- `store_pointer_in_structure`: Stores the value of a0 into offset 16 of the structure pointed to by a1, then runs straight into the next routine without returning.
- `set_serial_device_io_request`: The routine writes a constant byte to an offset in a structure pointed to by a0, stores a pointer to a global variable at another offset, and copies two caller-provided values into two further offsets.
- `set_serial_device_request`: The routine fills a request block in memory with a pointer to a global variable and several byte fields, then calls a function through a pointer obtained from a table.
- `init_serial_request_setup`: Initializes two request blocks by storing a byte value of 5 and a pointer at their respective offsets, then stores two handler addresses into the structure in a6.
- `call_library_vector_162`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `clear_and_call_library`: Clears d0 and branches to another routine that clears a global variable and calls a library function at offset 162 of a library base loaded from a global variable.

### Sounds2

- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Sounds3

- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Sounds

- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### translator.library

- `link_structure_via_pointers`: Reads a pointer from a global variable at offset 4, calls the routine stored at offset 84 of that pointed-to structure, and stores the result back into the structure at offset 34.
- `clear_flag_and_increment_counter`: Increments a 16-bit counter at offset 32 of the structure in a6 and clears bit 5 of the flag byte at offset 14 of that structure.
- `decrement_counter_call_on_zero`: Decrements a 16-bit counter at offset 32 of the structure in a6 and returns zero.
- `save_registers_check_flag`: Saves d3, a3 and a6 on the stack, copies a6 into a3, reloads a6 from the stack pointer, tests bit 5 of the word at offset 32 of a3 and if it is set sets that bit, then loads zero into d0 and jumps to the routine at 180.
- `unlink_node_and_call_library`: Unlinks a node from its doubly linked list by rewriting the next pointer at offset 4 and the previous pointer at offset 4 of the neighboring node.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `translate_buffer_markers`: Initializes a 26-byte work area on the stack with spaces and a zero terminator, stores the incoming pointer and counter, then repeatedly scans the input buffer for a '#' marker or a space and dispatches to helper routines based on which one is found, otherwise it returns.
- `copy_word_branch`: Copies the word in d0 into d1, then branches to a routine at offset 1b4 that calls a library routine, clears the byte just before the pointer held in the address register, and returns the low 16-bit value of the data.
- `clear_byte_and_return_word`: Calls a library routine, clears the byte just before the pointer held in the address register, then returns the low 16-bit value of the data register.
- `setup_translator_context`: Loads the pointer stored at offset 0 of the structure in a5 into a6 and returns if it is zero, then loads the longword at offset 0 of that pointer into a6 and calls two helper routines in sequence, then returns if the result of the second call is zero, otherwise it branches if the sign bit of d7 is set and falls through otherwise.
- `translator_dispatch_helper`: Loads a library base, sets up a context pointer, calls two helper routines, then branches based on a flag in d7 to further translation code.
- `call_four_branched_helpers`: Pushes the two pointers in a2 and a3 onto the stack, then calls four helper routines in sequence, each guarded by a branch that returns when the condition-code zero flag is set, before restoring a2 and returning.
- `pop_and_compare_byte`: Pops two pointers off the stack into a3 and a2, reads a byte from the address in a6 into d7, and if that byte equals 92 or 96 jumps to the routine at 514, otherwise jumps to the routine at 554.
- `stub_immediate_return`: Contains only a return in the decompiled output, so the routine does no visible work in the library.
- `match_bracketed_tag_text`: Skips forward through a text buffer until it finds an opening bracket character, then compares each following character against a reference string until it reaches a closing bracket.
- `compare_stack_byte_loop`: Decrements the loop counter in d6 and reads the byte at that offset from the table in a6 into d5.
- `set_ccr_bit_and_return`: Sets bit 2 of the condition code register and then returns immediately.
- `check_special_flag`: Loads one into a data register, calls another routine, and returns immediately when that routine reports success, otherwise it continues.
- `scan_string_for_equals`: Walks a byte string in a6, updating an accumulator register until it finds an equals sign and returns.
- `check_special_flag_ff`: Loads 255 into a data register, calls another routine, and returns immediately when that routine reports success, otherwise it continues.
- `check_backslash_or_backquote`: Copies the counter in d6 to d7 and decrements it, then reads the byte at that offset from the table in a6 into d0.
- `compute_byte_offset`: Computes a byte offset from a value in a5, adds a second value to it, and returns immediately when the result is not large enough, otherwise it stores a byte and pushes a word.
- `copy_counted_bytes_with_flag`: Copies a counted run of bytes from a source buffer to a destination buffer, decrementing the counter until it reaches zero.
- `truncate_text_at_break`: Walks backwards through a text buffer using a lookup table until it reaches a break character, then writes a zero byte at that position to end the string.
- `dispatch_on_special_character`: Searches a fixed table of special characters for the byte in the data register, then calls the handler at the matching position in a jump table.
- `load_byte_if_positive`: Tests a data register and returns immediately when it is negative, otherwise it loads a byte from a decremented pointer into another register.
- `index_table_from_byte`: Reads a byte from the source buffer, scales it by two, and uses it as an index into a table to load a word into the data register before returning.
- `call_check_bit6`: Calls a library routine and tests bit 6 of the status byte in d4, otherwise it returns.
- `wait_while_status_bit_set`: Calls a library routine once, then calls it again while bit 5 of the status value in d4 remains set.
- `check_status_bits_return`: Calls a library routine and tests bit 3 of the status byte in d4.
- `compare_char_set_flag`: Calls a library routine and tests bit 5 of the status byte in d4.
- `set_ccr_flag`: Sets bit 3 of the condition code register and returns.
- `compare_byte_and_branch`: Compares a byte against a set of letter constants and branches to a shared exit when a match is found, otherwise it returns without storing anything.
- `check_sequence_set_flag`: Calls a library routine and checks whether d1 holds the character N. When it does it calls the routine again and verifies d1 holds G, then checks bit 7 of the status byte and returns when it is clear.
- `set_ccr_bit_3`: Sets bit 3 of the condition code register and then returns.
- `load_byte_if_bit4_set`: Calls a library routine that loads a byte only when a data register is non-negative, then tests bit 4 of the status value and returns.
- `load_byte_if_bit2_set`: Calls a library routine that loads a byte only when a data register is non-negative, then tests bit 2 of the status value and returns.
- `load_byte_if_bit5_set`: Calls a library routine that loads a byte only when a data register is non-negative, then tests bit 5 of the status value and returns.
- `load_byte_if_bit8_set`: Calls a library routine that loads a byte only when a data register is non-negative, then tests bit 8 of the status value and returns.
- `poll_status_bit_until_clear`: Calls a library routine repeatedly while bit 5 of the status value stays set, then adds a register to a pointer and sets a condition code flag before returning.
- `load_byte_if_bit1_set`: Calls a library routine that loads a byte only when a data register is non-negative, then tests bit 1 of the status value and returns.
- `repeat_call_while_bit_one`: Calls a library routine repeatedly while bit 1 of the status value stays set, then adds a register to a pointer and sets a condition code flag before returning.
- `load_byte_if_bit7_set`: Calls a library routine that loads a byte only when a data register is non-negative, then tests bit 7 of the status value and returns without setting the flag when that bit is clear.
- `set_ccr_flag_2`: Sets bit 2 of the condition code register and returns.
- `normalize_and_store_byte`: Copies a byte from a source buffer into a structure, normalizes it into a printable range, and stores it while decrementing a counter, returning when the counter reaches zero.
- `write_space_before_pointer`: Decrements the address register and the counter, then writes a space character two bytes before the address register and returns.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 253 into d1, then returns.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies 32 to (a2)+, then copies 32 to (a2)+, then returns.
- `shift_buffer_for_marker`: Walks a buffer looking for a matching marker, then shifts the following bytes right by one and writes a marker character into the freed slot.
- `shift_buffer_insert_marker`: Shifts the bytes of a buffer right by one place until reaching a fixed position, writes a marker character into the vacated slot, stores an incremented pointer into a structure, and returns.

### version.library

- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `store_library_pointers`: Stores a 32-bit value from a register into a structure at offset 34 and another 32-bit value at offset 38, then returns.
- `increment_counter_and_clear_flag`: Increments a 16-bit value at offset 32 of a structure and clears bit 3 of a byte at offset 42 of the same structure, then returns.
- `decrement_counter_and_call`: Clears a data register, decrements a short counter at a fixed offset from a pointer and calls another routine when the counter reaches zero and bit 3 of a flag byte is set.
- `read_library_flag`: Saves registers, loads a pointer from a structure, and returns immediately when a flag is clear, otherwise it sets a flag and loads a zero value.
- `get_library_value`: Reads a 32-bit value from offset 38 of a structure, calls two routines through pointers in another structure, and returns the read value.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.

### Powerdrome (1989)(Electronic Arts)[cr Black Monks][t +4 Black Monks].adf:bootblock

- `initialize_bootblock_structure`: Initializes a structure by copying a table of words into it and filling the rest with fixed values, then repeatedly calls two library functions while waiting for a flag byte to change, and finally stores a pointer and a filename string into the structure before returning.
- `load_byte_if_positive_bit`: Loads a byte from a pointer and returns immediately when its high bit is set, otherwise it continues.
- `transform_and_store_buffer`: Walks a byte buffer, transforms each byte with two scaling values, and stores the results into a destination buffer while decrementing a counter until it reaches zero.
- `scale_and_shift_values`: Swaps the halves of a register, multiplies it by two other values, and arithmetic-shifts the two products right by six bits, then falls through to the next routine.
- `compute_and_store_offsets`: Compares two 16-bit values, and when they differ computes offsets and differences, then writes a series of values into a buffer at various offsets before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_globals_and_call_routines`: Initialises several global variables, then calls a helper and, if it returns zero, invokes two more routines before returning.
- `process_event_table`: Initializes a global variable to 20000 and repeatedly invokes two routines from a table using the values in d1 and d2, otherwise it returns.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `load_pointers_and_call_twice`: Calls a routine with a fixed address, loads two longword pointers from global variables, stores them into registers, and then issues two more calls before returning.
- `recursive_self_calls`: It invokes itself once and then calls two other routines in sequence before returning.
- `advance_pseudo_random_seed`: Multiplies the 16-bit seed stored at offset 98 of the structure in a6 by the constant 31351, adds one, and stores the result back as the new seed.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `install_interrupt_handlers`: Clears timing globals, installs interrupt handler addresses into the vector table, and adds interrupt servers through the exec library.
- `remove_interrupt_handlers`: Calls the two previously installed interrupt removal handlers, stores 1920 and 15 into the interrupt-enable and DMA-enable globals, then calls the five interrupt removal handlers.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `read_mouse_position_deltas`: It reads the mouse movement register and the fire-button port, computes the change in horizontal and vertical position since the last read, and accumulates clamped deltas into the state bytes.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `process_keyboard_input`: Reads the keyboard serial register, inverts its bits, and stores the result into a global byte.
- `check_global_equality`: Compares two fields of a global variable for equality and returns zero when they match.
- `start_blitter_dma_2`: It sets the two interrupt enable words, checks a counter and decrements it when nonzero, then sets both interrupt words again before returning.
- `start_blitter_dma`: Decrements the counter at the address 6a8cc by one, writes the value one with the enable bit set to the DMA control register to start the blitter channel, and returns.
- `check_copper_dma_counter`: Enables interrupts by writing to the interrupt enable register, then checks a global 16-bit counter that is decremented by start_copper_dma; when that counter is zero it branches to another routine, otherwise it disables interrupts by writing to the interrupt request register and returns.
- `start_copper_dma`: Decrements the counter at the address 6a8ce by one, writes the value two with the enable bit set to the DMA control register to start the copper channel, and returns.
- `start_audio_dma_2`: Enables the two audio DMA channels by setting their enable bits, then decrements a 16-bit counter in a global variable and starts the audio DMA by writing the enable bit to the DMA control register.
- `start_audio_dma`: Decrements the counter at the address 6a8d0 by one, writes the value four with the enable bit set to the DMA control register to start the audio channel, and returns.
- `decrement_and_start_dma`: Disables interrupts by setting bit 15 of the interrupt enable register, then decrements a 16-bit counter in a global variable and returns if it was zero, otherwise writes the DMA control value with the enable bit set to start the DMA.
- `start_chipset_dma`: Decrements the counter at the address 6a8d2 by one, writes the value eight with the enable bit set to the DMA control register to start the chipset channel, and returns.
- `ramp_audio_channel_volumes`: Checks the four enable bits of a global mask and, based on the sign of a shared volume level, decrements or increments it by one.
- `accumulate_elapsed_timer_ticks`: Reads the low timer byte of the CIA chip and stores it as the current reading.
- `allocate_even_sized_slice`: Adds one to the incoming size, rounds it up to the nearest even number, and adds that to a global base pointer.
- `allocate_even_sized_slice_2`: It stores a base address, then repeatedly calls a routine and, while the result is not positive, decrements the address and calls a series of other routines.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `setup_display_structures`: Allocates the display buffer and sets up its plane pointers, then repeatedly calls the graphics library to obtain a colour map and allocate the display structures, returning a negative value if any step fails.
- `load_artwork_if_flag_clear`: Tests a 16-bit state flag at a global offset and, when it is zero, loads two disk-path pointers and calls a routine, then sets the flag to one and clears the other before returning.
- `relocate_table_pointers_2`: Copies a base address into a scratch register, then loops twenty-nine times adding that base to every third longword of a table.
- `relocate_table_pointers`: Copies the base address into a scratch register, then loops twenty-nine times adding that base to every third longword of a table starting at the given pointer and advancing three longs each iteration before returning.
- `subtract_bias_from_bytes`: Reads a length from a helper, decrements it by one, then loops over that many bytes subtracting 128 from each byte of a buffer.
- `load_negative_one_and_branch`: Loads the value negative one into a data register, then unconditionally branches to a later instruction.
- `seek_and_read_file_pair`: It obtains a value from a helper and, when it is nonzero, records it and calls three routines at fixed offsets.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `invoke_table_on_handle`: Calls a helper to obtain a value and, when it is nonzero, stores it in a global variable, then invokes two routines from a table at a negative offset of that value twice more, calls another helper, and returns zero; otherwise it returns minus one.
- `check_marker_then_call_table`: Calls a helper to obtain a value and, when nonzero, stores it in a global variable and calls another helper, then, if the stored value equals an asterisk character, invokes a table routine at a negative offset twice, calls the table routine once more, calls another helper, and returns zero; otherwise it returns minus one.
- `return_negative_one`: Loads a fixed address into a pointer register, places minus one into the return register, and returns without touching any other data.
- `xor_scramble_data_block`: Loads a global pointer into a register, then loops two hundred and twenty times exclusive-oring each byte of the buffer with the key value and advancing the pointer before returning.
- `call_table_twelve_times`: Calls a helper to obtain a value and, when nonzero, stores it in a global variable, then invokes a routine from a table at a negative offset of that value twelve times in a row, calls the table routine once more, and returns zero; otherwise it returns minus one.
- `call_table_nine_times`: Calls a helper to obtain a value and, when nonzero, stores it in a global variable, then invokes a routine from a table at a negative offset of that value nine times in a row, calls the table routine once more, and returns zero; otherwise it returns minus one.
- `read_and_adjust_counted_table`: Calls a helper to obtain a value and, when nonzero, stores it in a global variable, then runs a loop of twenty passes that reads a four-byte field at an incrementing offset and adds the stored value to it, followed by two more loops of twenty passes each calling the table routine, then closes via the table routine and returns zero; otherwise it clears the global and returns minus one.
- `copy_padded_record_name`: Copies seven bytes from an indexed eight-byte record into a fixed buffer, converting spaces into zero bytes, then stores a helper value into a table slot selected by a second index.
- `store_helper_result_in_slot`: Calls a helper routine and, when the index register is nonnegative, stores the returned value into the table slot selected by multiplying that index by four and adding it to a global base.
- `copy_padded_name_and_load`: Copies seven characters from a record eight bytes long into a fixed buffer, converting space bytes into zero bytes, then calls a helper routine.
- `process_event_table_2`: Initializes a global variable to 20000 and repeatedly invokes two routines from a table using the values in d1 and d2, otherwise it returns.
- `route_pointer_dispatch`: Computes a pointer by subtracting the value at offset an address from the value at offset an address and stores the result at offset an address, then loads the address an address into a0 and jumps into another routine.
- `jump_into_routine`: Loads a fixed address into a0 and jumps into the middle of another routine, so it performs no work of its own before control leaves.
- `index_pointer_and_call_twice`: Multiplies a register by four to index a pointer table, loads the target pointer, calls a helper, then calls another routine before returning the original value.
- `invoke_table_pointer_chain`: Initializes a global variable to zero, then repeatedly calls a helper routine and returns early when its result is negative, otherwise continuing until the helper returns a nonnegative value.
- `process_command_codes_until_done`: Initializes three global variables, then repeatedly reads a code from a helper until it returns a negative value.
- `fill_four_by_sixteen_table`: Loops four times over rows and sixteen times per row, storing at each destination cell the row index times four plus a sixteen-bit value read from a source table by column, then returns.
- `store_registers_and_call`: Stores the two longwords passed in registers a1 and a2 into two consecutive global variables, then calls a routine and returns.
- `store_and_dispatch_pointer`: Stores two longwords passed in registers a1 and a2 into global variables, calls a routine, and then invokes a pointer stored at offset 36 or offset 132 of a structure depending on whether that value is zero.
- `load_long_and_call_routine`: Loads a long from an offset of a6 into a register and returns immediately when that value is zero, otherwise it loads a global pointer into a register, then calls a routine that returns a value which is stored at offset 2 of that global pointer.
- `branch_on_register_value`: Loads a long from an offset of a6 into a register and returns immediately when that value is zero, otherwise it loads a global pointer into a6, calls a routine through a negative offset, restores a different address into a6, clears a register, and returns; a tail call then jumps back to the start.
- `find_iff_chunk_index`: Loops over six four-byte table entries, comparing each byte with the corresponding byte of a tag at a fixed address and returning the matching index when all four bytes agree, otherwise returning minus one.
- `loop_four_steps_per_row`: Runs a loop that decrements a counter until it reaches minus one, and on each pass invokes four routines from a table in sequence, then calls a fifth table routine when a global value exceeds four, and finally returns.
- `unpack_packbits_or_copy_words`: When a flag is set it decodes a PackBits-style stream, copying literal runs and repeating single bytes for negative control codes, and otherwise it copies sixteen-bit words straight through, then returns.
- `call_routine_if_flag_clear`: Tests a longword flag at a global offset and, when it is clear, saves a pointer, loads a fixed address and a second pointer, calls a routine, and returns.
- `setup_and_call_loop_helper`: Loads the pointer stored at offset 6aa30 into a2 and copies a0 into a3, then sets up a counter of 40 in d6, a value of 200 in d7, and a pointer to 6a9a0 in a1 before writing the byte value 1 at offset 10 of that address.
- `call_helper_advance_offset`: Loads the address 66363 into a0 and the pointer at offset 6aa30 into a1, then calls the routine at 12b0.
- `copy_byte_then_word_loop`: Sets a counter to minus one, then loops copying a byte from a helper into the addresses pointed to by a0 and a1 while decrementing the counter until it reaches minus one.
- `clear_and_set_plane_pointers`: Invokes a table routine at a negative offset of a global, then, when a helper returns a positive value, stores that size into a structure at offset 8 and writes five successive base pointers into offsets 8 through 24 of the structure, each one stride further than the previous, and returns zero; otherwise it returns minus one.
- `init_and_run_table_routines`: Initialises two global variables to zero and four, then calls a helper and checks its result.
- `reconfigure_view_and_call`: Writes the value 65535 at offset 1fa of the structure in a6, then sets two global variables to one and five and a byte at offset 6a6a3 to five.
- `set_mode_one_reload_view`: Writes the value 65535 at offset 1fa of the structure in a6, then sets two global variables to one and five and a byte at offset 6a6a3 to five.
- `set_mode_zero_load_view`: Writes zero and four into two global variables, stores four at a related address, then calls the graphics library entry at offset two hundred and twenty-two through a pointer before returning.
- `update_frame_delta_and_flip`: Stores the current timer reading, computes the difference from the previous reading stored at offset 536, clamps that difference to a minimum of eight and a maximum of fifty, writes it at offset 540, then flips a flag at offset 506 to alternate between two sets of four stored values.
- `reload_view_clear_flag`: Calls the routine at 2fc2, loads the address 6a786 into a1 and the pointer at offset 6a66e into a6, then calls the table routine at offset -222 of that structure.
- `copy_display_state_and_reload`: Copies four saved values into matching globals, then, depending on a flag, calls one of two helper pairs that store different pointers and clear globals, then invokes four routines from a table at negative offsets in sequence, and finally returns.
- `reset_viewport_and_save_values`: Stores a fixed pointer into two globals, clears two more globals, then calls the graphics library entries at offsets two hundred and eight and two hundred and fourteen through the pointer before returning.
- `reset_viewport_into_second_pair`: Stores a different fixed pointer into two globals, clears two more globals, then calls the graphics library entries at offsets two hundred and eight and two hundred and fourteen through the pointer before returning.
- `call_routines_from_global_table`: Assigns a global variable to itself twice, then calls the routines stored at offsets 540, 216, 210, 402, 540, 564, and 562 of that same global variable, calling the routine at offset 564 again if two of its fields differ.
- `clear_flag_call_four_helpers`: Clears a 32-bit global variable to zero, then calls the helper routine at address an address four times in sequence.
- `sum_bytes_in_range`: Accumulates every byte in the program between a fixed start address and a fixed end address, incrementally advancing through the range, and returns the total in a register.
- `store_helper_result_offset`: Calls the routine at 3392 and stores its 32-bit result into the word at offset 14 of the structure in a6, then returns.
- `compare_and_dispatch_key`: Calls the routine at 3392 and compares its result with the value stored at offset 14 of the structure in a6.
- `clear_offset_24_structure`: Clears the 16-bit value at offset 24 of the structure in a6 and returns.
- `run_helper_when_timer_elapsed`: When a global counter is positive, a flag at offset 42 of a global is nonzero, a flag at offset 18 is clear, and either a counter reaches a global value or a force flag is set, it records the counter, clears the force flag, sets the busy flag to 65535, calls the helper at address an address, then clears the busy flag.
- `call_single_helper_return`: Calls the helper routine at address an address and returns.
- `enable_serial_interrupt_flag`: Sets bit 0 of the interrupt-enable register, clears a flag at offset 6a5f6, then writes 32769 to the interrupt-enable register to enable interrupts.
- `initialise_serial_port_settings`: Writes 185 to the serial period register and a status copy, clears four context fields, sets a byte at offset 146 of the structure in a6 to 255, clears four more fields, then writes 34817 to the interrupt-enable register to enable interrupts.
- `rotate_input_byte_buffer`: Maintains a rotating buffer of input bytes by storing each new byte at an incrementing offset that wraps past a limit, translating serial input into table entries and resetting state counters before returning a combined value.
- `store_word_and_branch`: Stores the value in register d1 at offset 51 of the structure in a6, then branches into the middle of another routine.
- `handle_key_input_state`: Subtracts 42 from the incoming byte in d0 and stores the result at offset 90 of the structure in a6.
- `clear_state_offsets`: Stores zero into the 16-bit values at offsets 50, 48 and 50 of the structure in a6 and stores 65535 into the 16-bit value at offset 51, otherwise it returns.
- `read_serial_and_queue`: Reads the input byte from the serial data register, masks it to a byte, and stores it into a keyboard queue indexed by a counter that wraps back to zero once it reaches eighty.
- `check_serial_buffer_pending`: Compares the read and write indices of a serial buffer, returning 65535 when they differ and zero when they are equal.
- `read_serial_byte_from_buffer`: Busy-waits until the read and write indices of a serial buffer differ, then returns the byte at the read index and advances the index with wraparound past 79.
- `read_and_store_serial_key`: Clears a byte at offset 144 of the structure in a6, reads one serial byte, and if it is a printable character stores it into a keyboard table while filling the queue at offset 140.
- `poll_until_helper_zero`: Invokes a helper up to 41 times, returning early the first time it yields zero and calling a second helper after each nonzero result, otherwise it returns.
- `clear_flag_copy_byte_add`: Clears a flag at offset 6a5f6, copies a byte from the address in a3 to offset 146 of the structure in a6, then adds 42 to that source byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_until_byte_zero`: Calls one routine in a loop and, while its byte result is nonzero, calls a second routine, returning once the result is zero.
- `store_position_defaults`: Stores the default 16-bit values 5, 3, 20, 175, 53 and 74 into six consecutive slots of the structure in a6, then copies the value at offset 434 into offset 438 and the value at offset 430 into offset 440.
- `push_registers_init_stack`: Pushes registers d2 through d7 and a2 through a6 onto the stack, loads the address of a global base into a6, writes the decimal value 65535 to the 16-bit field at offset 186, and returns to the caller.
- `initialize_globals_and_verify_text`: Initializes a large block of global variables to default values, then copies text from a source buffer into three destination buffers and compares a second text buffer against it.
- `index_and_call_parameter`: Loads a pointer from a global variable, scales the word at offset 112 of the structure in a6 by four, and indexes that pointer with the result, then sets the registers to 96, 63 and 1 and calls a routine before returning.
- `run_mode_sequence_branch`: Calls a setup routine and a paired-mode routine, then branches on the status byte at offset 340 of the structure in a6.
- `run_paired_mode_sequences`: Stores two fixed mode longwords and two global longwords into a structure, then calls five routines in sequence before storing a second set of mode and global values and calling two more routines only when a flag word is nonzero.
- `poll_input_and_clamp_values_2`: Polls input codes in a loop while waiting for the video frame, then dispatches on the code to increment or decrement several 16-bit counters at offsets 1100, 1102, 1104, 1106, 1110 and 1112 of the structure in a6, clamping each within its bounds, and advances a 16-bit counter at offset 616 when a flag bit is set.
- `advance_state_machine_counters`: Increments a 16-bit counter in a structure at offset 616 and, if a flag bit in a register is set, increments it again, then loops through a series of state-machine branches that adjust counters at offsets 1094, 1096, 1098, 1100, 1102, and 1090 and a counter at offset 1096, calling other routines and returning when a condition is met.
- `send_pair_of_block_commands_2`: Loads two fixed addresses into registers, stores a global longword and a fixed longword into a structure, then calls five routines in sequence before returning.
- `compute_scaled_bar_geometry`: Reads a table entry selected by a state word, derives two clamped values from another state field, and writes their scaled products divided by 35 into layout slots at offsets 434 and 436.
- `init_and_process_helper_values`: Checks the 16-bit value at offset 182 of the structure in a6 and the one at offset 820, returning if either is zero, then loads a table address and calls a helper routine.
- `sort_slots_by_word_key`: Initializes a table of four index-and-offset word pairs, then bubble-sorts those pairs in ascending order by the 16-bit value at offset 74 of each selected structure, and stores the leading index into the state word at offset 330.
- `copy_text_rows_into_buffer`: Calls four helper routines, then for four entries copies nine characters per entry from a text table into a display buffer, converting zero bytes into spaces, and finally writes a three-digit number with leading zeros stripped.
- `reset_and_process_helper_values`: Clears a 16-bit value at offset 820 of the structure in a6 and writes the value 5 at offset 618, then calls a helper routine once, seven times in a loop, and once more, and finally calls it five times while using its return value to branch.
- `conditionally_call_for_value_five`: Calls three helper routines, then calls a fourth routine only when a register argument equals five, and finally calls one more routine.
- `run_parameter_block`: Stores the constant values 1, 13, 10 and 113 into the state block at offsets 1122 through 1128, saves the mode longword, calls a routine under one mode constant and a second routine under another, then restores the mode longword and returns.
- `send_pair_of_block_commands`: Saves a mode value from a structure, sends two block commands using a state value and constant indices, restores the mode, calls two more routines, and finally restores the saved mode value.
- `forward_single_call_stub`: Loads four coordinate values into registers d0 through d3, sets d7 to zero, calls a span-drawing routine, and returns.
- `draw_two_digit_glyph`: Splits a value into its two decimal digits, stores them as characters, and when the tens digit is nonzero indexes a second font table to copy four glyph rows into the destination, shifting each row by a computed amount.
- `draw_double_vertical_span`: Saves the 32-bit value at offset 954 of the structure in a6, writes a global variable there, then calls the vertical span setup routine and the span drawing routine twice, restoring the saved value at the end.
- `pass_through_single_call`: Reads a coordinate from offset 1096 of the structure in a6, adjusts it and a second value, loads fixed coordinates into the remaining registers, calls a span-drawing routine, and returns.
- `init_span_registers`: Loads four constants into registers d0 through d3 and a zero into d7, then calls a routine to draw a vertical span within a screen area before returning.
- `call_three_keep_result`: Calls three routines in sequence and returns the data register value that was present on entry, unchanged.
- `set_blocks_and_call`: Stores the values 166, 172, 52 and 90 into the words at offsets 686 through 694 of the structure in a6, stores 13, 161, 17 and 166 into the words at offsets 1122 through 1128, then calls two routines and returns the data register value present on entry unchanged.
- `set_blocks_call_alt`: Stores the values 109, 115, 52 and 90 into the words at offsets 686 through 694 of the structure in a6, stores 13, 104, 17 and 109 into the words at offsets 1122 through 1128, then calls two routines and returns the data register value present on entry unchanged.
- `clamp_and_scan_table_bounds`: Clamps two 16-bit values to fixed ranges, stores them, then loops over a table of nine entries comparing a computed value against stored bounds and jumps to the matching entry when found.
- `return_max_int`: Loads the constant an address into register d0 and returns it, then loads the constant 12345 into d0 and returns it.
- `clear_flag_after_two_calls`: Saves a register on the stack, calls two routines with different constant indices, then clears a flag word in a structure and restores the register before returning.
- `set_flag_after_two_calls`: Saves a register on the stack, calls two routines with different constant indices, then sets a flag word in a structure to all ones and restores the register before returning.
- `poll_input_and_clamp_values`: Loops while waiting for the video frame and reading input codes, setting a flag word at offset 194 of the structure in a6 to 1 for code 14 or to -1 for code 28, then clamps two global values to the ranges 0 to 359 and 0 to 374 and checks them against a table of 11 entries.
- `classify_input_code`: Calls a routine that reads input and clamps values, then checks the result in d0: if it equals 81 it jumps to another routine, if it equals 113 it also jumps there, and if it is between 1 and 11 it converts the value by subtracting 48, extends it to 16 bits, and jumps to a third routine, otherwise it jumps to that third routine.
- `return_constant_4`: Returns the constant -1 in d0 to the caller without any other effect.
- `process_table_entry_when_ready`: Scales an incoming index by eight and reads two words from a table at that offset, then, when the flag word at a further offset is positive, saves a global base, calls a routine with the two words, restores the base, calls another routine, and finally restores the saved base.
- `load_addresses_jump`: Loads two fixed addresses into a0 and a1, then jumps into another routine, so it only sets up those registers before handing off.
- `save_call_restore_longword`: Checks a scaled value against zero, then saves a longword, calls a routine with two table words, calls it again with no arguments, and restores the saved longword before returning.
- `jump_into_routine_with_tables`: Loads two table addresses into registers and jumps into the middle of another routine to continue its execution.
- `copy_words_and_jump`: Copies the word at offset 694 of the structure in a6 into d1 and the word at offset 696 into d2, then jumps to another routine.
- `copy_text_buffer_bytes`: Computes a destination address from a global variable and copies bytes from a text buffer into it until it encounters a space, a carriage return, or a terminator, then returns.
- `copy_text_line_lowercase`: Copies characters from a text stream into a buffer until it hits a carriage return, null, space or the byte 3, then converts remaining printable characters to their lowercase form and forwards each one to a font-glyph routine.
- `read_uppercase_line_input`: Clears a sixteen-byte buffer to spaces, then reads characters in a loop, converting letters to uppercase and storing them until a carriage return or the buffer fills.
- `call_library_descending_thirteen_times`: Makes thirteen calls through a library jump pointer, passing a counter that counts down from 12 to 0, then restores the registers and returns.
- `adjust_clamped_structure_values`: Saves a global base value, then adjusts two 16-bit values at offsets 246 and 248 of the structure in a6 (each clamped to 225 and shifted right by 4) by calling another routine with the decrease or the complement of the change, and finally restores the saved global base value.
- `update_offset_1a0_value_2`: Saves a word, calls a routine with the difference, then clamps another word and updates a counter, calling a routine when the value decreases or when it is small.
- `update_offset_1a0_value`: Reads a word at offset 1a0 of the structure in a6, overwrites it with a value from d6, computes the difference, then saves a global base, calls a routine with the difference, calls another routine, and restores the base before returning.
- `xor_shift_store_word`: Reads two state words, computes their exclusive-or, shifts the result left by two bits, and stores the outcome into a third state word.
- `and_bitplanes_in_blocks_2`: Reads a state field and derives three source pointers, then loops over sixteen 32-byte blocks writing the bitwise AND of paired words into a destination block.
- `and_bitplanes_in_blocks`: Clamps a signed state field to a range, derives two source pointers, then writes the bitwise AND of paired words into a destination block using an inner loop of 12 copies and an outer loop of 11 blocks.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `clear_bitplane_backwards`: Writes zero longwords backwards from the end of a 32000-byte block using an unrolled loop of 40 stores, repeating the block 100 times.
- `zero_block_backwards_by_count`: Reads a state index from offset 1160 of the structure in a6, multiplies it by two, and uses it to fetch a base pointer from a table indexed by a global variable, then adds the value at offset 954 of that structure.
- `call_indirect_via_pointer`: Performs one indirect call through a routine pointer stored at offset -30 of a global structure, then returns.
- `advance_pseudo_random_state`: Multiplies a 16-bit value stored at offset 98 of the structure in a6 by the constant 31351, adds one, stores the low word back, and leaves the full multiply result in d0.
- `build_row_offset_table`: Allocates a table through a routine and fills it with running multiples of forty, then allocates a second table and stores both pointers in global words, then returns.
- `dispatch_jumptable_index_2`: Computes the smaller of two incoming values, adds offset-derived amounts to a base, then dispatches to a routine chosen from a jump table indexed by a register.
- `fetch_jumptable_index`: Reads a word from a jump table indexed by d7, stores d5 into d6, inverts d6, and loads the next index from the table, then falls through into the following routine.
- `dispatch_jumptable_index`: Jumps indirectly into a jump table indexed by d7, so it only dispatches to another routine and never executes the following instructions.
- `clear_plane_words_unreached`: ANDs a mask into the matching word of all four bitplanes, then clears every word from the next position up to an end pointer in a loop, then ANDs a second mask into the matching words of another four bitplanes.
- `and_mask_all_planes_word`: ANDs a mask from d6 into the matching word of all four bitplanes pointed to by a0, then returns.
- `or_first_plane_mask_others`: ANDs a mask from d6 into the matching words of the first three bitplanes and ORs a value from d5 into the first bitplane, then returns.
- `modify_buffer_bits`: ANDs a mask from d6 into the matching words of the first two bitplanes and ORs a value from d5 into the matching words of the last two bitplanes, then returns.
- `draw_span_in_bounds`: Returns unless both coordinate pairs lie inside a 320 by 200 area; otherwise it calls a routine, then loops incrementing one coordinate until it reaches a limit, adjusting two other values by 40 each iteration and calling a routine inside the loop.
- `draw_font_glyph_at_position`: For characters between 20 and 91, computes a glyph location in a font table and copies nibbles from its rows into bitplane words at a coordinate, skipping other characters, then returns.
- `check_coordinate_bounds`: Copies d1 and d2 into d0 and d1, subtracts five from d1, scales d0 by four, adds three to the result, then calls a routine to check whether the coordinates fall within the visible screen area before returning.
- `validate_vertical_line_bounds`: Sets d7 to three, copies d1 and d2 into d0 and d1, subtracts one from d3 and five from d1, scales d0 by four, adds three to the result, then calls a routine to validate the vertical line bounds before returning.
- `draw_font_glyph_variant`: When the character code falls between 20 and 91, it indexes a second font table and copies four glyph rows into a destination, shifting each row left by a bit count derived from the second argument.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `set_flag_then_call`: Sets bit 0 of a 16-bit flag at offset 506 of the structure in a6, then calls the routine at a fixed address and returns.
- `clear_flag_then_call`: Clears bit 0 of a 16-bit flag at offset 506 of the structure in a6, then calls the routine at a fixed address and returns.
- `call_library_twice`: Loads a base address from a global variable into a5, calls the routine at 15306, reloads another global base into a5, calls the same routine again, and returns.
- `run_four_calls_sequence`: Makes four routine calls in sequence and returns.
- `handle_blit_result_twice`: Calls a helper routine to obtain a value, and when that value is positive it calls another routine and returns, otherwise it calls that routine again and returns.
- `adjust_and_call_blit`: Loads a pointer from a global variable, adds 256 to it and follows it, copies the words in d1 and d2 into d0 and d1, sets d7 to 1, calls a routine, restores d1 and d2, adds 27 to d1 and 55 to d2, then returns.
- `wait_for_video_frame`: Tests a 16-bit flag at offset 34328 and, when it is zero, loads a second address into a0 and calls the graphics library function at offset 402 of the base in a6 before returning.
- `copy_sixteen_words_to_buffer`: Reads a source pointer from a data table, then copies sixteen 16-bit words from that block into a fixed buffer using a loop that decrements a counter.
- `copy_thirty_two_words`: Reads a source pointer from a data table, then copies thirty-two 16-bit words from that block into a fixed buffer using a loop that decrements a counter.
- `copy_palette_records`: If a global variable is nonzero it copies four longwords at a time from a source pointer into a table for three iterations and then calls a routine, otherwise it copies eight consecutive longwords and calls a routine, always returning the incoming longword.
- `call_palette_then_routine`: Invokes a routine located at a fixed offset in a global structure, then calls a second routine and returns.
- `call_graphics_function`: Calls the graphics library function at offset 192 of the base in a6, then calls the routine at a fixed address and returns.
- `copy_palette_record`: Loads a pointer from a global variable and copies a palette record of three longwords, a word and four more longwords from a source pointer into it, then calls a routine and returns.
- `clear_palette_block_by_flag`: Clears either eight or sixteen longwords from a block, chosen by a flag, and makes a routine call on the first path.
- `load_hardware_colour_registers`: Copies four 16-bit values into the colour register bank at offset a fixed address, repeating the block four times before restoring registers and returning.
- `save_longword_rows_when_enabled`: When a flag is nonzero, copies four longword rows from a source block into a destination pointer using a loop, otherwise copies eight longwords directly, then returns the original value.
- `fade_colour_nibbles_step`: Steps through a buffer of 4-bit colour values, decreasing each nonzero value by one and setting a flag when any value changes, repeating the scan while the flag is set.
- `save_longword_rows_via_call`: Loads a pointer from the stack at offset 12 into a1, sets a2 to a fixed constant, calls a routine to save longword rows, then jumps to another routine.
- `repeat_calls_while_counter_nonzero`: Tests a global word and optionally calls a subroutine, then runs a loop of four subroutine calls that repeats while a 16-bit counter is non-zero.
- `load_view_after_two_calls`: Calls two helper routines, then invokes the graphics library function at offset 4 through the base pointer stored in a global variable, and returns.
- `invoke_view_after_call`: Calls another routine, then invokes a routine through the graphics library base stored in a global variable, and returns.
- `step_packed_nibbles_toward_target`: Walks two byte buffers backwards from offset 30 to 0, nudging each low nibble of the second buffer one step toward the matching nibble of the first buffer and moving the high nibble one step only on odd offsets, then stores the recombined byte back.
- `process_and_update_palette`: Loads a pointer from a structure in a6 and a constant address, calls a routine, sets d7 to thirty-one, calls another routine, and when the result is nonzero it calls a further routine before returning.
- `clear_word_at_offset_702`: Writes zero to the 16-bit value at offset 702 of the structure in a6, then returns.
- `clear_four_bitplane_rectangle`: Reads the top row and left column from structure fields, computes the row and column counts, and looks up the row start address in a table.
- `call_indirect_from_vector_2`: Follows a pointer stored at a negative offset in a global variable and jumps to the routine it references, then returns.
- `call_indirect_from_vector`: Follows a pointer stored at offset -30 of a global variable and jumps to the routine it references, then returns.
- `clip_rectangle_to_bounds`: Reads a rectangle's extents and four bounds from two structures, clamps the horizontal and vertical sizes against those bounds, and reduces the offsets accordingly.
- `blit_helper_result_twice`: Calls a helper routine to obtain a value, passes that value to a graphics-library routine through a global base, then makes a second call to the same base and returns.
- `reset_audio_volumes_and_wait`: Calls four helper routines, then zeroes the shared global word four times and loops on a fixed-count wait.
- `forward_single_helper_call`: Loads two pointers from a structure into registers, calls a helper routine, and returns without keeping any state of its own.
- `start_audio_channel_dma`: When the flag at offset 756 of the structure in a6 is zero, it sets up one audio channel's pointer and length registers in the audio device, waits for the display frame twice through the graphics library when a shifted mask is non-zero, and writes the channel's DMA and interrupt enable bits.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_run_loop`: Initializes a global structure by storing fixed values at its offsets, then clears two memory blocks starting at a fixed address and a fixed address by looping until a counter reaches zero.
- `clamp_and_fill_glyph_table`: Computes a value from a structure field and a source byte, clamps it between 16 and 144, and stores it back.
- `process_string_bytes`: Loops over a zero-terminated string, and for each character calls a helper routine that shifts a byte left eight times while accumulating bits, then advances to the next character until the terminator is reached.
- `rotate_and_accumulate_bytes_2`: Loops over eight bytes, and for each byte shifts it left while looping eight times and calls a helper routine whenever a shifted bit was previously set, then returns two registers unchanged.
- `rotate_and_accumulate_bytes`: Builds an index from a register and a base pointer, then loops over a count of bytes reading each source byte.
- `zero_31_word_block`: Computes a destination index from two registers and a base pointer, then loops 31 times writing zero to each location and advancing by 11 words.

### Powerdrome (1989)(Electronic Arts)[cr Black Monks][t +4 Black Monks].adf:pd.bin (raw code file, load address unknown)

- `clear_globals_and_jump`: Clears the low byte of a global variable, clears another global variable, and jumps to the routine at the fixed address.
