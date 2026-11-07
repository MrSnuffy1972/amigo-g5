# Game 2272 (castlevania (1990)(konami)[cr qtx][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02272). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901648 top-level bytes in 129 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 621 of 621 named (0 author symbols), 621 of 621 described (37 by rule, 406 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1843 of 1843 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 4683 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 19962 of 19963 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 290360 |
| code | 66288 |
| data | 484884 |
| padding | 29908 |
| unused | 30208 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| type | 2 | 117 | 2 |
| Dir | 2 | 117 | 2 |
| Echo | 2 | 117 | 2 |
| Execute | 2 | 117 | 2 |
| Copy | 2 | 117 | 2 |
| Makedir | 2 | 117 | 2 |
| wait | 2 | 117 | 2 |
| disk-validator | 1 | 386 | 1 |
| ram-handler | 2 | 10 | 1 |
| serial.device | 4 | 982 | 84 |
| Castlevania (1990)(Konami)[cr QTX][a].adf:cast | 3 | 17706 | 470 |
| Castlevania (1990)(Konami)[cr QTX][a].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 50 | 50 |
| BAMMER (raw code file, load address unknown) | 1 | 0 | 0 |
| VAN0 (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:art/magic: high-entropy data file (7.63 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:art/DEMON: high-entropy data file (7.83 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:art/frog: high-entropy data file (7.79 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:art/FRANKIE: high-entropy data file (7.74 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:art/DEATH: high-entropy data file (7.79 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:art/goodies: high-entropy data file (7.64 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:art/SCAST: high-entropy data file (7.62 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:art/dracula: high-entropy data file (7.77 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:art/GHOUL: high-entropy data file (7.84 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:art/DOG: high-entropy data file (7.82 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:art/MUMMY: high-entropy data file (7.87 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:art/CASTSHPS: high-entropy data file (7.90 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/VAN11: high-entropy data file (7.73 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/van12: high-entropy data file (7.84 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/VAN13: high-entropy data file (7.92 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/VAN14: high-entropy data file (7.90 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/van15: high-entropy data file (7.90 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/van16: high-entropy data file (7.92 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/VAN17: high-entropy data file (7.83 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/VAN18: high-entropy data file (7.91 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/van19: high-entropy data file (7.91 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/van20: high-entropy data file (7.93 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/van21: high-entropy data file (7.89 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/VAN22: high-entropy data file (7.91 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/VAN23: high-entropy data file (7.91 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/VAN24: high-entropy data file (7.94 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/VAN0: high-entropy data file (7.88 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/VAN25: high-entropy data file (7.92 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/VAN1: high-entropy data file (7.94 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/VAN26: high-entropy data file (7.90 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/van2: high-entropy data file (7.90 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/VAN27: high-entropy data file (7.92 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/VAN3: high-entropy data file (7.90 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/van28: high-entropy data file (7.86 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/van29: high-entropy data file (7.93 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/VAN4: high-entropy data file (7.92 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/VAN5: high-entropy data file (7.82 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/van6: high-entropy data file (7.81 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/van7: high-entropy data file (7.89 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/van8: high-entropy data file (7.90 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/VAN30: high-entropy data file (7.91 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/VAN31: high-entropy data file (7.92 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/van9: high-entropy data file (7.82 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/VAN32: high-entropy data file (7.92 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/VAN33: high-entropy data file (7.92 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/VAN34: high-entropy data file (7.90 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/van35: high-entropy data file (7.92 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/VAN36: high-entropy data file (7.93 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/van37: high-entropy data file (7.88 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/VAN10: high-entropy data file (7.89 bits/byte), classified as packed data
- runtime segment an address: 33 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:art/BAMMER: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- castlevania (1990)(konami)[cr qtx][a]/Castlevania (1990)(Konami)[cr QTX][a].adf:screens/VAN0: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 2 never-loaded region(s), 4683 bytes, recorded as UNVERIFIED DATA (under 1% of bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `fatal_error_exit` (cast): Shows a fatal message (the copy-protection failure text for code 99, otherwise a numbered error) in the game's message box and leaves the program. Kind: password/code-wheel check. Pass path: the branch that avoids the call to u10__fatal_error_exit.

## Functions


### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_buffer_from_offset`: If a destination pointer is non-zero, it copies sixteen long words from a source area located at an offset from a register to the destination, advancing both pointers each iteration.

### Dir

- `find_max_and_prepare_buffer_2`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header and a sequence of words into a buffer, sets up a stack frame, and copies sixteen long words between two areas using another routine.
- `copy_16_words_from_table`: The routine copies sixteen 32-bit words from a table located at an offset from a pointer into a caller-supplied buffer, then calls another routine.

### Echo

- `find_max_and_prepare_buffer_3`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header and a patterned table into a buffer, copies a block of data from a source area to a stack area and back, and returns a status code.
- `copy_sixteen_words`: If the output pointer is non-zero, the routine copies sixteen 32-bit words from a source located at an offset from a register into the output buffer and then calls another routine.

### Execute

- `max_entry_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header and a sequence of words into a buffer, copies a small block of data to and from a destination pointer, and returns a status code.
- `copy_table_and_call`: The routine copies sixteen 32-bit values from a table located at an offset from a pointer to a caller-supplied buffer, then calls another routine.

### Copy

- `find_max_and_prepare_buffer_4`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header and a patterned block into a buffer, sets up a stack frame with a self-reference, and copies sixteen long words between two buffers via another routine.
- `copy_buffer_from_table`: The routine copies sixteen 32-bit words from a table located at a fixed offset from a pointer in a2 into a caller-supplied buffer, then calls another routine.

### Makedir

- `find_max_and_prepare_buffer_5`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header and a sequence of words into a buffer, sets up a stack frame with a self-reference, and copies sixteen long words between two areas via another routine before returning a status code.
- `copy_header_block`: The routine copies sixteen 32-bit words from a table located at an offset from a pointer into a caller-supplied buffer, then calls another routine.

### wait

- `find_max_and_prepare_buffer_6`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header and a patterned block into a buffer, sets up a stack frame with a sentinel and a self-reference, and then iterates over a table while invoking another routine repeatedly.
- `copy_16_words_from_table_2`: The routine copies sixteen 32-bit words from a table pointed to by a register offset by a fixed amount into a caller-supplied buffer, then calls another routine.

### disk-validator

- `initialize_disk_validator`: The routine sets up a set of pointers and counters in a global structure, writes a digit character derived from a global counter into a buffer, clears and initializes several tables used for validation, and then enters a loop that prepares per-entry masks and shifts for a checksum-like processing.

### ram-handler

- `dispatch_indirect_jumps`: The routine calls a function obtained from a global variable and then calls an indirect jump table entry.

### serial.device

- `return_constant`: The routine returns the constant a fixed address without performing any other operation.
- `initialize_serial_device_state`: The routine clears several fields in a device structure pointed to by a register, copies a value from a register into a field and into multiple other fields, and writes shifted versions of that value into two further fields.
- `check_flag_and_call_handler`: The routine tests a byte at an offset from a pointer in A1 against a mask and, if the test passes, calls a handler routine obtained from a table via A6.
- `set_byte_and_test_flag`: The routine stores 253 at an offset from a pointer in A1 and, if a byte at another offset from the same pointer is clear for mask 81, calls a handler obtained from a structure pointed to by A6.
- `disable_interrupts_and_call`: The routine sets a flag in a byte at an offset from a register, writes a value to a global variable, calls another routine, writes a different value to the same global variable, and clears the flag in the byte.
- `handle_serial_message_completion`: The routine clears a flag byte in a structure pointed to by A1 and updates global serial device state depending on the message type.
- `clear_flag_and_update_structure`: The routine clears a flag byte in a structure pointed to by A1 and updates a pointer field in a global structure when A1 matches a stored pointer.
- `copy_structure_and_update_flags`: If a flag byte in the structure pointed to by A1 is set, the routine copies the structure’s first word through a pointer stored in the structure, updates a counter in A6, modifies flag bytes in the structure, stores 254 at an offset, and conditionally calls a handler from A6.
- `conditional_structure_update`: The routine checks a pointer in A6 against A1 and, on a match with specific conditions, updates a global variable and clears flags; otherwise it may copy structure fields and decrement a counter.
- `decrement_counter_and_clear_flag`: The routine checks a flag byte in the structure pointed to by a1 and, if set, copies the structure’s first word through a pointer stored in the structure, updates a pointer stored at an offset from that word, decrements a short counter in a6, modifies two flag bytes in the structure and clears a byte.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `reset_serial_device_state`: The routine checks a flag in the device structure and returns early if set.
- `invoke_library_vectors`: The routine calls several library vectors obtained from a structure pointed to by A6, copies a word from the structure through a pointer, and returns a value from the structure.
- `link_serial_device`: The routine clears fields in a structure pointed to by A1, manipulates flag bytes, updates counters and pointers in a structure pointed to by A6, links the structure into a list, writes to a global variable, and touches INTENA and INTREQ.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `register_serial_device`: The routine links a serial device structure into a global device list and updates a device counter.
- `handle_serial_device_interrupt`: The routine processes a serial device control block pointed to by A1, updates several fields in the block and in a linked structure, manipulates flag bytes and a counter, reads a byte from CIAB_PRA via a helper, combines values to form a status word stored in a global variable, and signals completion by writing to a global flag.
- `decrement_counter_and_signal`: The routine decrements a short counter in a structure and, if it reaches zero, writes constants to fields of a message structure, clears a flag byte, clears a field, and calls a handler obtained from a table.
- `forward_serial_handler`: The routine simply calls another routine and returns, acting as a thin wrapper.
- `handle_serial_interrupt_state`: The routine updates serial device fields and a counter in a structure, masks and sets flag bytes, combines a value from a helper routine with a bit derived from a structure byte, stores the result into a global variable, and increments two counters in the structure.
- `update_serial_interrupt_state`: The routine compares an input value with a global counter and updates global serial device fields, masks and sets flag bytes in a structure.
- `process_serial_data_and_signal`: The routine combines a value from a helper routine with a bit derived from a byte in a structure, stores the result into a global variable and updates counters.
- `dequeue_structure_and_update_flags`: The routine writes the constant 1 to a global variable and returns when a word in a global structure is non-zero.
- `write_global_flag`: The routine writes the constant 1 to a global variable and returns.
- `check_serial_interrupt_status`: Reads a byte from a global variable and combines it with a value from a hardware register to update a global status flag.
- `initialize_serial_device_3`: The routine writes constants to fields of a structure pointed to by a register, clears flag bytes in a global structure, and conditionally calls handlers obtained from a table.
- `copy_state_and_test_flag`: The routine calls another routine, then tests a byte at an offset from a pointer in A1 against mask 81 and, if clear, calls a handler obtained from a structure pointed to by A6.
- `copy_serial_state`: The routine copies a 32-bit value from an offset within a structure pointed to by a register to two other offsets in the same structure, then clears a 32-bit field at a third offset.
- `dispatch_serial_command`: The routine sets a status bit in a global byte when a pointer is non-zero, optionally calls a handler and then conditionally invokes a handler routine obtained from a table before returning a value.
- `clear_status_and_call_handler`: The routine clears a bit in a byte field of a device structure, conditionally calls another routine based on a non-zero word in the structure, writes a value to a global variable twice, and if a masked byte from a pointer is zero it calls a handler obtained from a table in the structure.
- `write_globals_and_test_flag`: The routine writes two constants to a global variable, calls another routine, and then tests a byte at an offset from a pointer in A1 against mask 81, calling a handler from A6 if the test passes.
- `clear_serial_flag`: The routine calls another routine twice and then clears a 32-bit field in a structure pointed to by an address register.
- `iterate_and_call_routine`: The routine repeatedly dequeues a structure pointed to by a pointer in A0, updates the pointer field, and calls another routine for each dequeued structure until the pointer becomes null.
- `empty_stub_3`: The routine contains no instructions and returns immediately.
- `set_serial_flags_and_maybe_call`: The routine sets a flag byte in a structure pointed to by a1 and writes a constant to the next byte.
- `update_device_state`: The routine reads a global byte, tests two flags from a structure pointed to by A6, conditionally sets a bit in the byte, stores masked bytes and words between the A1 structure and the A6 structure, and calls a handler obtained from a table if a flag in the A1 structure is clear.
- `register_serial_device_2`: The routine clears a bit in a byte of the A1 structure, tests a flag and a counter, and if set links the A1 structure into a global list, updates a counter and a global variable, clears a flag, and returns; otherwise it sets a flag in the A6 structure, calls another routine, stores the returned value, and writes to a hardware register.
- `set_flag_and_call_routine`: The routine sets a bit in a byte located via a pointer, calls another routine, stores a value returned in A1 into a word at an offset from the same pointer, and writes a constant to a global variable.
- `clear_serial_flag_and_disable_interrupts`: The routine clears a flag in a byte field of a structure pointed to by a1, and if a condition on that byte and a short field is met it sets another flag in the same byte, clears a pointer field, writes a value to a global variable, and calls another routine via a pointer stored in the structure.
- `set_global_flag_and_configure_timer`: The routine writes the constant 1 to a global variable and, when a byte in a global structure is not equal to a constant, clears a word in the structure, clamps an input value, writes timer-related words to the structure, and calls a routine via a vector obtained from the structure.
- `empty_routine_4`: The routine contains no instructions and returns immediately.
- `clear_serial_flag_and_update_status`: The routine clears a flag byte in a structure, conditionally writes values to a global variable, calls another routine, and sets a byte in a device structure.
- `configure_serial_device_handler`: The routine clears a flag byte in a structure, conditionally writes values to a global variable, calls another routine, and sets a byte in the input structure.
- `configure_device_and_call_handler`: The routine writes a value to a global variable, calls another routine, writes a different value to the global variable, and if a masked byte from a pointer is zero it calls a handler obtained from a table in a device structure.
- `configure_serial_device_parameters`: The routine copies several bytes and words from a structure pointed to by A1 into fields of a global serial device structure, updates status bytes, calls helper routines to initialize state and compute divisor and status, and stores additional values from the structure.
- `compute_serial_status_word`: The routine builds a 16-bit status word from a byte in a structure and a flag in another structure, then stores the word into a global variable.
- `decrement_indexed_table_lookup`: The routine checks a flag byte in a structure pointed to by a register and conditionally decrements a short index.
- `write_byte_to_structure`: The routine stores a constant byte at an offset from a pointer in A1.
- `compute_serial_divisor`: The routine computes a divisor from a value in the A6 structure using a constant that may be changed by a character test, stores the result in a field of the A6 structure and a global variable, clears a flag, and conditionally sets bits in the global variable and a flag based on a character and a flag test.
- `compute_serial_divisor_and_flags`: The routine divides an input value by the low word of another input and stores the result in a structure field, copies that field to a global variable, clears a flag bit in a byte field, and conditionally sets a high bit in the global variable and a flag bit in the byte field based on two structure bytes.
- `initialize_serial_device_6`: The routine obtains a device structure via a library call, writes fields in the structure with a pointer, a library base, a size constant and a pointer to another routine, then installs empty handler pointers into the structure and sets a global variable before returning success.
- `initialize_serial_device_5`: The routine calls several empty or setup routines, writes constant bytes and pointers into a structure pointed to by a register, installs handler pointers by calling another routine and storing the returned values, and sets a global variable to a constant before returning a constant.
- `initialize_io_request`: The routine stores the A0 pointer into a field of the A1 structure, writes constants and pointers into fields of the A0 structure, including a pointer to a global variable and values from A3 and A6, and returns.
- `set_serial_device_io_request`: The routine writes a constant byte to an offset in a structure pointed to by a0, stores a pointer to a global variable at another offset, and copies two caller-provided values into two further offsets.
- `set_serial_device_request`: The routine fills a request block in memory with a pointer to a global variable and several byte fields, then calls a function through a pointer obtained from a table.
- `initialize_serial_device_4`: The routine initializes fields in a serial device structure, sets up request blocks with pointers and constants, installs handler pointers, calls library vectors via a library base, and writes to INTENA to control interrupts.
- `call_library_vector_162`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `empty_stub_4`: The routine contains no instructions and returns zero.
- `enqueue_serial_request`: The routine clears a field of the A1 structure, tests a request field and counters, conditionally clears and sets a flag in the A1 structure, links the A1 structure into a global list, increments a counter, and calls a handler obtained from a table; otherwise it calls a handler if a flag in the A1 structure is clear.
- `dequeue_serial_messages_2_2`: The routine drives a serial device structure in A1 and a per-message structure, repeatedly dequeuing a message pointer and updating device fields.
- `dequeue_serial_messages_2`: The routine drives a serial device structure in A6 and a per-message structure in A1, repeatedly dequeuing a message pointer and updating device counters and flags.
- `process_serial_message_3`: The routine drives a serial device structure in A6 and a per-message structure in A1, clearing device flags, updating message state bytes and counters, and repeatedly dequeuing pending messages from a linked list.
- `process_serial_message`: The routine drives a serial device structure in A6 and a per-message structure in A1, clearing device flags, updating message state bytes and linking the message into a device list.
- `process_serial_input`: The routine processes a byte from D0 using a serial device structure in A6 and a per-message structure in A1.
- `return_immediately`: The routine is empty.
- `identify_routine`: This routine is not empty; it contains ten instructions but the decompiler has not produced meaningful pseudo-code.
- `serial_message_dequeue_loop`: The routine initializes a per-message status field and clears flags in the serial device structure, then repeatedly dequeues a message pointer from a queue.
- `dequeue_serial_messages`: The routine initializes a per-message status field and clears flags in the serial device structure, then repeatedly dequeues a message pointer from a linked list, updates device counters and flags, and processes the message data byte by byte using a circular buffer.
- `process_serial_message_2`: The routine initializes a per-message status field and clears flags in the serial device structure, then repeatedly dequeues a message pointer from a queue, updates device and message flags, and processes bytes from a circular buffer using the serial device structure.
- `handle_serial_device_status`: The routine examines a flag word in a device structure and, if negative, sets a global variable, updates a status byte and conditionally copies a pointer field before invoking a handler obtained from a pointer in the structure.
- `process_serial_buffer_2`: The routine checks a flag byte in a device structure and conditionally clears a flag and sets a global variable, then returns.
- `process_serial_buffer`: The routine checks a flag byte in a device structure and conditionally sets a flag and a pointer field, then calls a handler obtained from a field in the structure.
- `process_serial_byte`: The routine updates fields in a device structure based on an input byte, advances a buffer pointer and a counter, may wrap the pointer, sets a flag byte, and then calls a handler obtained from a table in the structure.
- `handle_serial_input`: The routine conditionally sets a flag byte and a pointer field in a device structure, then either processes an input byte by storing it through a pointer, updating a counter and possibly wrapping the pointer, or skips processing based on a status byte.
- `initialize_serial_device_2`: The routine sets a flag in the A6 structure, copies a value from one field to another if a field is zero, and calls a handler obtained from a table.
- `process_device_buffer`: The routine checks a field in a device structure and, if a condition holds, updates a pointer and a counter in the structure and may copy a pointer value, then calls another routine and invokes a function pointer from the structure.
- `initialize_serial_device`: The routine checks a flag in a device structure and, if clear, copies a value from one field to another, then calls another routine and invokes a function pointer stored in the structure.
- `update_serial_status`: The routine copies a byte from one field to another in a device structure, clears a bit in a status byte, conditionally modifies a global variable, and sets the global variable to a constant when a condition on a status byte holds.
- `update_device_status`: The routine copies a byte from one field to another in a device structure, sets a bit in a status byte, conditionally modifies a global variable, and may clear a byte field and return early based on a flag in the structure.
- `clear_status_byte`: The routine stores zero to a byte offset in a device structure and returns.
- `initialize_device_flags`: The routine tests a flag byte in a device structure and, if clear, sets the flag, clears a field, computes a value from another field, stores it, writes a constant word, and calls a function pointer from the structure.
- `clear_serial_flag_2`: The routine clears a bit in a flag byte in a device structure, conditionally sets another bit and a pointer field, and may invoke a handler obtained from a table in the structure.
- `set_serial_device_flags`: The routine reads a byte from a serial device structure and updates status flags in the same structure.
- `set_serial_interrupt_flag`: The routine sets a flag in the device structure and copies a value from one field to another if a field is zero, then calls a handler obtained from a table.

### Castlevania (1990)(Konami)[cr QTX][a].adf:cast

- `clear_buffer_and_initialize_io_2`: The routine obtains a value from another routine, clears a block of memory in a global variable by writing zeros in a loop, stores a register value into a global pointer, copies a global variable into a field, and conditionally calls a handler from a table.
- `reset_and_run_game_loop`: The routine initializes several word fields in a global structure, installs a set of function pointers and then enters an infinite loop that repeatedly invokes those indirect routines and other helpers, updates flags and counters in the global structure, and conditionally calls additional indirect routines based on the flag values.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `loop_until_threshold_met`: The routine repeatedly calls a function obtained from a table via a global pointer and compares its return value with a global variable.
- `clear_two_word_tables`: The routine clears a set of word entries in two adjacent tables by writing zero to each entry in a loop of thirty-two iterations.
- `clear_tile_table`: The routine clears a rectangular block of 16-bit entries in a global table.
- `clear_table_entries`: The routine clears a table of eighteen 16-bit entries by writing zero to each element.
- `clear_enemy_state_words`: The routine clears a block of fourteen 16-bit words in a global variable by storing zero to each consecutive location.
- `init_system`: Start-up: stores the copy-protection routine in a function pointer for the main routine to call, opens the libraries, installs the interrupt handlers, the audio and keyboard handlers, suppresses DOS requesters and loads the common sounds and graphics.
- `clear_state_flags`: The routine clears a block of ten consecutive word-sized fields in a global variable, then calls another routine, and finally sets two word-sized fields in the same global variable to one.
- `initialize_game_state`: The routine writes constants to several global variables and then loads a byte from a table indexed by a short offset, scales it, and stores the result in a global word.
- `copy_word_from_offset`: The routine calls another routine twice and then copies a 16-bit value from one offset of an address derived from a register to another offset of the same base.
- `initialize_global_state`: The routine writes constants to a global variable and two other global variables, then loads a byte from a table indexed by a short stored in a global variable into a global variable, and loads a second byte from a second table using an index derived from another global variable and the first loaded value into another global variable.
- `reset_game_state_flags`: The routine writes zero and one values to a set of word locations in a global structure and then calls another routine twice.
- `set_sprite_attributes`: The routine copies a byte from a table indexed by a short offset stored in a global variable into a word field of a global structure, then sets several word fields of that structure to constant values and a flag field to one, choosing between two values for one field based on the sign of the offset.
- `initialize_game_state_2`: The routine clears a number of word-sized fields in a global state structure, sets a few of them to one and one to a negative value, increments a counter field, and then calls another routine.
- `reset_game_state`: The routine writes zero to several word-sized fields in a global structure and sets two other word fields to one, then calls another routine.
- `set_title_screen_art`: It exits with a flag set when a condition is met or when the cursor passes a threshold, then cleans up by calling other routines.
- `clear_state_flags_2`: The routine calls two other routines via pointers stored in memory relative to a base pointer, then clears five 16-bit words in memory relative to the same base pointer.
- `copy_state_fields_and_load`: The routine copies a block of bytes from one region derived from a register to another region derived from the same register, then copies several word and long word values from a set of source offsets to a set of destination offsets also derived from the register.
- `load_castle_dat_fields`: It copies several word and long word values from one set of offsets derived from a register to another set of offsets derived from the same register.
- `return_zero_constant`: The routine is empty and returns a constant zero.
- `load_castle_dat_and_copy`: It calls another routine with a global variable and an offset, then stores a constant word.
- `save_game_key`: Developer key in the game: when the player has energy left it shows SAVING GAME and saves the current game state through the save routine.
- `toggle_collision_flag`: If two words at offsets from a register are non-zero, it checks a word flag and sets it to one or zero, then calls another routine with a global variable and a constant.
- `set_collision_off_flag`: The routine stores a constant into a global variable and then calls another routine with a global variable and a constant.
- `toggle_game_time_flag`: If two words at offsets from a register are non-zero, it calls another routine with a global variable and a constant, then sets a word flag to one or zero depending on its previous value.
- `set_game_time_on_flag`: The routine calls another routine with a global variable and a constant, then stores a constant into a word at an address derived from a register.
- `toggle_life_count_flag`: If two words at offsets from a register are non-zero, it calls another routine with a global variable and a constant, then sets a word flag to one or zero depending on its previous value.
- `set_life_count_on_flag`: The routine calls another routine with a global variable and a constant, then stores a constant into a word at an address derived from a register.
- `reset_game_state_2`: The routine writes a constant to a global variable and then calls two other routines, one with a global variable and a constant as arguments and one with no arguments.
- `set_screen_number`: The routine checks two global flags and, if set, calls another routine to write three values and calls another routine to fill a small stack buffer.
- `load_stage_enemy_art`: Loads the enemy shape files of the current stage: for each of up to nine entries of the stage table it builds the file name from the art directory and the enemy name, loads and decodes the file and stores a pointer to each animation frame it holds.
- `call_indirect_via_offset`: The routine loads an address computed from a global variable plus a constant offset and calls the routine found there.
- `load_castle_shapes_and_goodies`: It then calls another routine with argument zero.
- `reset_three_flags`: The routine clears three global flags by testing each and writing zero if non-zero.
- `read_screen_table_entry`: The routine calls a handler to fill a temporary buffer and then calls another handler with the input parameter.
- `reset_game_state_3`: The routine clears several global variables, iterates over a table of 17 entries resetting fields and clearing related entries, then clears a small table, loads a byte from a table via an index, calls another routine, and conditionally calls other routines with constants, moves values between globals, and updates a counter.
- `tick_animation_counters`: The routine increments a frame counter and, when it reaches a threshold, resets it and decrements several delay counters stored in global variables.
- `clear_state_variables`: The routine clears several word-sized fields in a global structure and sets two word fields to one.
- `read_player_controls`: Reads the joystick each frame: separates the two fire buttons into press edges (a held button does not repeat) and turns the direction bits into the player's move, jump and crouch requests.
- `call_indirect_from_table`: The routine loads a function pointer from a table using an offset from a global variable and calls another routine with the passed argument, returning its 16-bit result.
- `call_and_return_result`: The routine calls another routine with the incoming parameter, calls a second routine with no arguments, and returns the result from the first call unchanged.
- `save_previous_positions`: Stores the current scroll position, the player's position and the position of each of the sixteen object slots as their previous values, for the next frame's redraw.
- `clear_flags_for_entries`: The routine iterates over a table of entries and clears two 16-bit fields for each entry whose second field equals four.
- `add_and_call_table_function`: The routine reads two short values from a global variable, adds them, and returns the sum if the sum is zero or a flag condition is met.
- `adjust_target_offset_and_fetch_value`: The routine increments a target offset in a structure by eight when a global flag is set, then adds two short values from global memory.
- `dispatch_table_call`: The routine checks a parameter against a range and, if in range, calls a function pointer taken from a table indexed by the parameter.
- `call_indirect_via_table`: The routine loads a function pointer from a global table using an index derived from the argument and an offset, then calls the retrieved routine with a value taken from another global table indexed by the same argument.
- `recursive_input_handler`: It calls another routine with a word taken from an offset, and if a flag word is non-zero it calls the same routine again with the same argument.
- `process_input_event`: It computes a value from several words in a table entry indexed by a parameter, then conditionally calls a table function via an indirect pointer and another indirect routine, and finally calls another indirect routine with the result if non-zero.
- `update_table_entry`: If a flag in a global variable is set, the routine calls a function pointer twice, stores the second result in a global variable and increments a counter if the first result is non-zero.
- `process_input_events`: The routine clears pending input flags, builds a 16-entry table from a global table using offsets and masks, calls another routine with that table, then iterates the entries and conditionally calls another routine for each.
- `increment_counter_and_check_table`: The routine loops a number of times given by a parameter, increments a word at a fixed offset from a base register each iteration, checks a byte in a table addressed by a global base pointer plus an index derived from two words, and stores either zero or the incremented word back to the same word location.
- `increment_and_test_flag`: The routine repeatedly increments a word at an offset from a pointer and checks a byte in a table addressed by a global base pointer plus an index derived from two words from the pointer.
- `increment_word_and_test_table_byte`: The routine repeatedly increments a word at an offset from a pointer and checks a byte in a table addressed by a global base pointer plus an index derived from the pointer.
- `decrement_counters_and_call_helpers`: The routine loops twice, each time decrementing a global counter until a helper returns non-zero or the counter reaches zero, calling other routines in between.
- `synchronize_scroll_and_state`: The routine copies a word from a global variable to another global variable, calls another routine through a function pointer stored in a global variable and writes its result back, then restores the original value if the result is zero.
- `process_level_tilemap`: Iterates over a tilemap buffer using a base pointer and row offsets, invoking a drawing routine for each cell and dispatching additional calls based on tile values and masks.
- `initialize_display_buffers`: The routine calls two routines via a table of function pointers, then iterates a nested loop calling a routine via a table of function pointers with an index argument, then calls another routine, and finally calls three more routines via a table of function pointers.
- `run_callback_grid`: The routine calls a setup routine through a pointer stored in memory, then invokes a callback routine for each entry in a 40 by 40 grid via a function pointer, and finally calls three cleanup or finalization routines through pointers stored in memory.
- `redraw_status_panel`: Redraws the status panel: the score, the player and enemy energy bars, the time, the hearts, the lives and the stage number, through the panel printing routines.
- `copy_value_and_loop_call`: The routine copies a short from one offset of a base pointer to another offset of the same base pointer.
- `redraw_status_loop`: It reads a word from an offset, adjusts it, and then calls another routine in a loop either ten times starting from zero or from the adjusted value up to fifteen.
- `parse_script_bytes`: The routine walks a byte table using an index derived from a global variable and a loop count, reads three-byte groups and skips values 0, 1 and 2.
- `scan_tile_map_for_events`: The routine iterates over a range of rows defined by a global counter and walks a three-row tile map using a stride stored in a global variable.
- `set_flag_by_index`: The routine selects one of five offsets from a base pointer using a switch on a parameter and stores the value 1 as a word at the chosen offset.
- `clear_flags_and_call`: The routine clears a specific word-sized global flag selected by a field of the input parameter and then calls another routine with a constant argument, returning its result.
- `check_state_flag`: The routine loads a word from a global variable and returns 1 if its value matches one of several listed constants, otherwise it returns 0.
- `increment_counter_and_update_offset`: The routine checks a global variable and a shifted global variable against thresholds and a zero flag.
- `decrement_timer_or_update_counter`: The routine checks a global variable and a shifted global variable and a flag.
- `evaluate_hitbox_condition`: The routine tests several global words for zero or non-zero conditions and, depending on the outcome, either returns 0, 4, 7 or 8, adjusting a global word by adding or subtracting a constant and calling another routine when required.
- `evaluate_hitbox_condition_2`: The routine checks a set of global words for zero or non-zero conditions and, based on the results, returns 0, 3, 5 or 6, modifying a global word by adding or subtracting a constant and invoking another routine in the non-zero branches.
- `adjust_offset_and_branch`: The routine examines a few global words, adds a constant to a global word in several branches, calls another routine when a condition holds, and returns 0, 6 or 8 depending on the tested flags.
- `adjust_offset_and_branch_2`: The routine tests a few global words, subtracts a constant from a global word in multiple paths, calls another routine when a condition is met, and returns 0, 1, 5 or 7 according to the flag checks.
- `process_state_flags_and_update`: The routine loads a word from a global offset, tests several bits and another global word, and conditionally updates several global words and calls other routines with constant or computed arguments.
- `process_animation_step`: The routine checks a global flag and a global counter and, if set, calls another routine with a combined value from three global words.
- `reset_game_state_on_condition`: The routine checks a global flag and the bits of an input parameter, and if the condition is met it clears a global word, sets another global word, calls another routine, sets a further global word, calls another routine with a constant argument, stores the returned word into two global words, and clears one of them.
- `decode_tile_attribute`: The routine computes an index from two parameters, looks up three bytes in a table using a global base pointer and offsets, and conditionally writes those bytes to a global short or combines them into a return value based on two global flags.
- `set_tile_collision_flags`: The routine writes constants to several global variables and, when the parameter is not -1, loads values from a table into globals using an index derived from the parameter and a lookup.
- `load_animation_frame_data`: The routine clears a block of animation-related words in a global structure, computes an index from a global timer and a global offset, selects a table entry using a base pointer, a stride and the computed index, and then copies three consecutive words from the table into a set of global fields in a fixed order, with a conditional extra copy and a minimum index clamp.
- `process_enemy_hitbox_table`: Iterates over a table of enemy entries and updates per-entry timers and hitbox flags.
- `reset_input_state_flags`: The routine clears a block of input-related word flags and then recomputes them from several global input state words, setting flags when particular bits are set or cleared.
- `update_animation_state_2`: The routine updates several word-sized global variables that appear to track animation timing and flags, increments and resets counters, clears and sets flag words, loads a byte from a table using an index derived from global values into a word, and calls other routines under various flag conditions.
- `set_tile_flags_from_map`: The routine reads a short entry from a table indexed by a parameter and uses it to compute two map offsets.
- `process_tile_collision`: The routine updates a per-entity collision state and computes a 32-bit value from a tile map and entity data.
- `update_tilemap_marker_state`: The routine selects an entry in a table using fields from its argument and a global stride, then checks a marker flag and a byte in a second table.
- `update_enemy_state`: The routine updates a per-entity slot in a table using fields from the entity and a global base, computes a 32-bit value from a tile map and entity data, and then conditionally modifies collision, marker and flag fields, calls another routine with a derived parameter, and updates position and state words in the slot based on table entries and random values.
- `update_enemy_state_table`: The routine updates a small table of three entries by comparing two short fields per entry, setting a flag and adjusting a value, then calls another routine conditionally based on a global flag and iterates over a set of sixteen structures to invoke another routine twice and possibly change a status field, and finally clears two global flags and two entries in a table if a flag is set.
- `search_tilemap_for_marker`: The routine computes a pointer into a table using two fields from the first argument and a stride from a global variable, then walks the table entry by entry while comparing a high byte of the sum of the two arguments to an index.
- `insert_or_update_event_entry`: The routine scans a fixed-size table of ten entries for a matching record based on three fields derived from its arguments and a global flag.
- `process_input_entity_slot`: The routine scans a small table of entity slots for an entry matching a parameter, marks the entry, looks up a type from a global table and either triggers a special handling path for a particular type or creates a new entity via another routine.
- `accumulate_and_test_range`: The routine adds an incoming integer to a global variable and reads a value from a function pointer.
- `process_entity_collision`: The routine iterates over entity slots and checks a set of flag and type conditions to select a primary entity and a secondary entity.
- `execute_collision_checks`: The routine computes a vertical and horizontal interval from a global table entry and a global position, then iterates over ten objects, testing their bounding boxes against the interval and calling another routine for matches.
- `clear_entity_slot`: The routine clears a set of bytes in two entity-related structures derived from a table entry and then invokes another routine several times with values computed from that entry.
- `process_entity_state`: The routine processes an entry from a table of entity records indexed by the parameter.
- `scan_table_for_value`: The routine iterates over a table of sixteen short entries spaced seventy bytes apart in memory relative to a base pointer.
- `store_format_string_argument`: The routine passes a format string reference to another routine together with a global variable and a value read from an address derived from an uninitialized register.
- `handle_bad_data`: The routine contains no valid instructions and ends immediately.
- `call_other_routine_with_global`: The routine calls another routine, passing zero and the address of a global variable, then returns.
- `format_two_digit_number`: The routine calls another routine with the constant 8 and the address of a global variable.
- `format_two_digit_value`: The routine calls another routine with a constant and a pointer to a global variable, then returns.
- `call_other_routine_with_global_2`: The routine calls another routine, passing zero and the address of a global variable, then returns.
- `call_indirect_via_table_2`: The routine checks a global variable and, if non-zero, uses it as an index into a table of pointers to call another routine indirectly with a derived pointer argument and a constant.
- `check_state_and_call_handler`: The routine reads a short value from a global variable.
- `recursive_self_call_2`: The routine consists of a single self call followed by a return, so it invokes itself recursively without performing any other data operation.
- `recursive_self_call`: The routine consists of a single self call followed by a return, so it invokes itself recursively without performing any other data operations.
- `call_indirect_from_table_2`: The routine loads a function pointer from a table via an offset from a global variable and calls it with a pointer to another global variable and a constant argument.
- `conditional_indirect_call`: The routine checks a field of its parameter against a constant and, if the condition holds, performs an indirect call through a global function pointer using a global variable as argument and a constant as second argument.
- `call_indirect_via_table_3`: The routine loads a function pointer from a table at an offset from a global variable and calls another routine with a global variable and a constant argument.
- `call_indirect_via_table_4`: The routine loads a function pointer from a table using an offset from a global variable and calls another routine with a global variable and a constant argument.
- `find_or_create_entity_slot`: The routine searches a table of entity slots for a matching entry based on several fields from the parameters and a global table, skipping slots marked inactive or with a specific state, and returns a zero result when a match is found.
- `update_object_entry`: The routine calls another routine and, if the result is non-zero, computes an offset from a global variable and a scaled index, then writes several fields in the resulting structure: a shifted value from the parameter, a scaled value derived from the parameter, and clears a field while copying one field to another within the same structure.
- `clear_entity_flags`: The routine calls another routine with a constant and a parameter, then uses the returned short to compute an offset into a global variable.
- `select_enemy_sprite_and_reset_state`: The routine selects a sprite index based on a global flag and several tables and then calls another routine with that index.
- `set_entity_flags`: The routine calls another routine with a fixed first argument and the two parameters it received.
- `set_entity_flags_from_index`: The routine calls another routine with a fixed first argument and the first parameter, then checks the returned short.
- `decrement_counter_and_update_struct`: The routine checks a global counter and a table entry against a global limit.
- `process_entity_action`: The routine dispatches on a type field from the first argument and, for several types, calls another routine with derived parameters and clears or updates a set of per-entity slots in a global structure using the returned index.
- `compute_entity_physics`: The routine computes a per-entity offset from a global base and updates several words in the entity slot using values read from a table and from three random bytes.
- `clear_enemy_state`: The routine calls another routine with a constant argument and, if the result is non-zero, computes an offset from a global variable and the result, then writes several 16-bit values into that computed location, copying one field from within the same structure and loading one field from another global location.
- `map_enemy_sprite_to_state`: The routine reads a room index from a global variable and a table, then uses it as a selector.
- `map_enemy_sprite_to_state_2`: The routine compares a register value with a constant and returns a constant if they match, otherwise returns zero.
- `return_zero_2`: The routine is not empty; it contains eight instructions that end in a return of the constant zero.
- `return_zero`: The routine returns the constant zero without performing any other work.
- `process_enemy_spawners`: The routine checks several global state flags and, when conditions are met, calls other routines to create or update enemy objects with parameters derived from global variables and then copies flag words.
- `clear_enemy_slot`: The routine clears a per-entity slot in a table when the entity is not in a specific state.
- `check_table_for_value`: The routine iterates over a table of short values and calls another routine for each entry that is not equal to a specific constant.
- `iterate_table_and_call`: The routine loops over eleven entries in a table referenced via a global variable, computes each entry’s base address, checks three short fields for specific values and a non-equal value, and calls another routine for entries that satisfy the conditions.
- `process_object_type`: The routine adjusts an object type field using a table lookup and then dispatches based on the type value.
- `check_entity_collision_flag`: The routine derives an entity slot address from a parameter and inspects two tables indexed by fields from that slot.
- `fetch_byte_from_table`: The routine computes an address in a table using the first parameter and an index from the second parameter.
- `load_castlevania_title_assets`: The routine builds a table of pointers to five text resources and then repeatedly draws the title screen, waiting for input, and animates the title graphics by writing to screen memory via a blitter helper.
- `store_two_word_pairs`: The routine stores the low and high 16-bit halves of the first argument into two consecutive 16-bit fields of a global variable, and stores the low 16-bit half of the second argument into the next 16-bit field of the same global variable.
- `update_enemy_state_table_2`: The routine iterates over eight 10-byte entries in a table pointed to by param_1 and updates per-entry timers and counters.
- `initialize_village_art_and_audio`: The routine clears several global variables, loads two blocks of art data referenced by the text strings and builds a table of fourteen word values from those blocks, then configures audio and writes the table entries to a display list using a helper routine.
- `process_input_loop_2`: The routine repeatedly calls indirect routines using offsets from a base pointer and updates a counter, exiting with a short value after a loop.
- `process_input_loop`: The routine repeatedly calls indirect routines using offsets from a base pointer and updates a counter, exiting with a short value after a loop condition is met.
- `poll_input_with_timeout`: The routine repeatedly calls an input routine with a constant argument and decrements a counter until the input is non-zero or the counter reaches zero.
- `call_three_routines_via_table`: The routine calls three other routines through a table indexed by an uninitialized value.
- `insert_score_and_render`: The routine loads handlers from a table, checks a file via another routine and either clears a ten-entry score table and its name strings or loads the table from SCORES.DAT.
- `shift_score_table_and_render`: The routine shifts a table of ten 32-bit entries down one position starting from the insertion point, copies associated name strings, and renders each entry’s score and name using text output calls.
- `handle_score_file`: The routine manipulates a table of scores in memory, writes formatted text using referenced strings, and calls routines for file access and fatal error handling.
- `initialize_quartex`: The routine calls an indirect routine with a global variable and a fixed address.
- `set_animation_state_and_loop`: The routine clears a global word and, if a global short is non-zero, sets two global words and a step value based on the sign of that short, then loops updating a global short and calling three indirect routines each iteration.
- `run_animation_sequence_2`: The routine sets several global words, then loops a number of times updating a global short and calling three indirect routines each iteration, followed by a fixed loop that updates another global short and calls indirect routines.
- `run_animation_sequence`: The routine updates a global short in a loop driven by two uninitialized values, calling three indirect routines each iteration.
- `run_reverse_animation_sequence_2`: The routine sets global words based on a global short, loops updating a global short and calling three indirect routines, then runs a fixed loop updating another global short and calling indirect routines.
- `run_reverse_animation_sequence`: The routine sets two global words, then loops a number of times taken from a global short, updating a global short and calling three indirect routines each iteration.
- `update_animation_state`: The routine repeatedly updates a global short by adding a scaled value, then calls three indirect routines and decrements a counter until it reaches zero.
- `call_indirect_table_entry`: The routine loads a function pointer from a global table using an index built from two global short values and a byte from another global table, then calls that function with an argument derived from a global short value.
- `reset_globals_and_step_2`: The routine writes constants to several global words, copies values between global words, and loops updating a global short while calling three indirect routines.
- `reset_globals_and_step`: The routine writes constants to several words in two different address spaces and clears a set of global words.
- `accumulate_scaled_value_loop`: The routine repeatedly reads a short from a table pointed to by a pointer, scales it and adds it to a short stored in a global variable, then calls three other routines via function pointers stored in a global table.
- `reset_state_and_wait`: The routine clears several global variables, sets up a small set of state words, then calls a series of other routines to perform initialization.
- `reset_and_run_intro_sequence_2`: The routine clears several fields in a global structure, then runs a timed loop that repeatedly calls indirect routines to poll input and update the display.
- `reset_and_run_intro_sequence`: The routine clears several fields in a global structure, then runs a timed loop that repeatedly calls indirect routines to poll input and update state, incrementing counters and invoking a tile-map scan.
- `process_input_and_update_state`: The routine repeatedly calls an indirect routine twice with values loaded from global words and a global structure, then enters an inner loop that calls two indirect routines, increments a counter, checks the counter, calls an indirect routine for input and branches, optionally calls another indirect routine, updates a global short and calls an indirect routine, and conditionally calls an indirect routine again.
- `wait_for_input`: The routine calls indirect routines to process input, updates a counter, and returns after clearing several global words when a condition is met.
- `reset_game_state_4`: The routine calls two other routines via pointers stored in a table and clears three 16-bit fields in a global structure, setting them to zero.
- `select_and_spawn_entity`: The routine reads several values from a global variable base, computes a derived position and a room identifier, then uses a room-type selector from a global table to choose an entity index through a series of range checks on the derived values and a flag.
- `spawn_entity_from_table`: The routine selects an entity type based on a set of range checks on several input values and a flag in a global variable, then if the selected type is enabled in a table it marks the entry and calls another routine to obtain a slot.
- `create_entity_slot`: The routine checks a table entry, marks it, calls another routine to obtain a slot, and writes several fields into a structure at a computed offset.
- `initialize_game_state_3`: The routine sets a flag byte, copies a value between address spaces, calls another routine with a global variable and stores the result, then calls another routine.
- `initialize_game_state_3_2`: The routine sets a flag byte, copies a value between address spaces, calls another routine with a global variable and stores the result, the.
- `call_other_routine`: The routine consists of four instructions that unconditionally call another routine and then return.
- `handle_game_event`: The routine sets a flag byte, copies a value between two address spaces, calls another routine with a global variable and stores the result, then calls another routine with three arguments and conditionally calls further routines and stores a constant and a computed value.
- `handle_game_event_no_flag`: The routine copies a value between two address spaces, calls another routine with a global variable and stores the result, then calls another routine with three arguments and conditionally calls further routines and stores a constant and a computed value.
- `set_timer_and_callback`: The routine stores a value returned by another routine into a global variable, then calls another routine with a fixed argument and a flag.
- `build_effect_script_table`: The routine builds a script table for sound effects using a helper routine and a set of string tables.
- `check_and_dispatch`: The routine tests a field of its parameter against a set of constant values.
- `handle_music_event`: The routine processes a music event parameter and updates a global state table.
- `poll_input_until_nonzero`: The routine repeatedly calls a function through a pointer taken from a global variable until the returned short is non-zero, then returns.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `fatal_error_exit`: Shows a fatal message (the copy-protection failure text for code 99, otherwise a numbered error) in the game's message box and leaves the program.
- `toggle_sound_flag`: The routine toggles a global short flag and, if another global flag is non-zero, calls another routine with a value taken from a global variable.
- `set_sound_off_flag`: The routine stores a constant into a global variable and, if a global flag is non-zero, calls another routine with a value taken from a global variable.
- `toggle_effect_flag`: The routine toggles a global short flag and, if another global flag is non-zero, calls another routine with a value taken from a global variable.
- `clear_effect_flag`: The routine sets a global variable to a constant and, if a global flag is non-zero, calls another routine with a value taken from a global variable.
- `pause_screen`: Pause screen: shows the pause banner (the text the Quartex crack put there), waits for a key and switches the developer cheat keys on when the cheat code was typed meanwhile.
- `display_pause_message`: The routine checks a short value on the stack and either calls a routine with two constants or calls another routine with a stack buffer and a constant then calls a routine with the buffer and a constant, followed by a call to another routine.
- `empty_stub_2`: The routine contains two instructions and is empty in effect; it is referenced by data and called by initialize_memory_manager and calls fatal_error_exit.
- `handle_copy_protection_failure`: The routine checks a short value on the stack.
- `handle_copy_protection_failure_2`: The routine checks a short value on the stack.
- `set_flag_and_repeat_handler_2`: The routine checks a value returned by another routine and, if it is large enough, calls three handlers from a table, sets a global flag, and then runs a series of repeated handler calls that copy data between temporary buffers using table-driven parameters.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `initialize_memory_manager`: The routine stores an address in a global variable and writes several offsets derived from an allocation returned by another routine into a table at a fixed negative offset from a register.
- `set_global_word_and_dispatch`: The routine calls another routine, stores the constant 4 into a word at an offset from a base pointer, calls another routine twice, then calls a function pointer taken from an offset from the same base pointer.
- `copy_handler_table_entries`: The routine copies sixteen 16-bit entries from a table selected by the low byte of the argument to two destination tables and then calls another routine with a pointer to one of the destination tables.
- `swap_word_tables`: The routine copies 32 words between two global tables depending on a parameter, clears a set of words in a global table, and conditionally calls another routine with a global address.
- `clear_buffer_from_source`: The routine checks a global variable and, if non-zero, calls another routine with its address.
- `format_signed_number`: The routine is eight instructions long, is referenced by data and called by pointer in hunk 1 data and format_string_with_args, and calls format_number_with_sign.
- `format_number_with_sign`: The routine formats a signed 32-bit value into an ASCII decimal string using another routine for digit extraction, writing the sign if needed and terminating the string with a null byte.
- `signed_32_by_16_divide`: The routine divides a signed 32-bit dividend in d0 by a signed 16-bit divisor in d1 and returns a signed 48-bit result with the quotient in the high 32 bits and the remainder in the low 16 bits.
- `abs_small_value`: The routine receives a signed 16-bit value in a register, flips its sign if it is negative, and returns the absolute value as a 16-bit result.
- `absolute_value`: The routine receives an integer parameter and, if it is negative, replaces it with its negation.
- `random_below`: Returns a pseudo-random number from zero up to one less than the given limit, stepping the game's random generator.
- `seed_random`: Seeds the random generator from the CIA time-of-day and timer counters.
- `format_string_with_args_2`: The routine parses a format string containing percent directives and writes formatted values into a stack buffer, using another routine to fetch arguments.
- `format_string_with_args_3`: The routine parses a format string containing percent directives and writes formatted values into a stack buffer, using another routine to fetch arguments from a table.
- `format_string_with_args`: The routine parses a format string containing percent directives and writes formatted values into a stack buffer, using another routine to fetch arguments from a table and to output characters.
- `conditionally_call_another_routine`: The routine loads an unsigned byte from a global variable, adds a constant offset and checks the signed result against a negative threshold.
- `count_string_length`: The routine receives a pointer to a byte string and walks it byte by byte until it finds a zero byte, counting how many bytes precede the terminator.
- `blend_tile_into_planes`: The routine blends a source tile from a table into three destination buffers using per-plane masks and a per-plane operation flag, writing the resulting bytes into five bitplane offsets for each of the four iterations.
- `apply_tile_mask_to_planes`: The routine writes masked bytes to three byte pointers derived from a base offset and two global offsets, using a mask table indexed by a register value and two bit masks from global variables.
- `draw_char`: Common body of the two character routines: looks the character up in the font and writes its eight lines into each of the five bitplanes, setting or clearing the bits according to the foreground and background colour masks.
- `copy_word_pair`: Copies two 16-bit words from one pair of offsets in a global variable area to another pair of offsets in the same area.
- `copy_word_pair_to_global`: The routine copies a 32-bit value from the argument into two 16-bit words of a global variable, or copies two 16-bit words from that global variable to itself when the argument is negative.
- `set_up_int_server`: The routine obtains a handle from a global table, installs the same handler in a set of interrupt structures and repeatedly signals them until a call returns zero.
- `reset_game_state_5`: The routine clears several 16-bit fields in a global structure by writing zero and a full-mask value, then calls another routine twice before returning the combined value of the two input registers.
- `initialize_object_state`: The routine copies a 16-bit field from the argument into several 16-bit slots of a global structure, derives a mask by shifting the stored value right by three and clearing the low bit, writes that mask to two slots and clears two other slots, then calls another routine and returns the value it received in D0.
- `toggle_flag_bits`: Xors a 16-bit word at a fixed offset in a global variable area with a constant mask.
- `store_word_from_param`: Stores the low 16 bits of a parameter into a 16-bit word at a fixed offset in a global variable area.
- `store_word_from_parameter`: The routine stores a 16-bit value taken from the first parameter into a global variable via a pointer derived from a register.
- `copy_palette_block`: It writes the high and low halves of the source to each buffer in an interleaved pattern for seven iterations, advancing the offset by eight each time.
- `reset_dma_control`: The routine clears several words in a block pointed to by a register, writes a constant to a word, copies a long from a global location to another location, calls another routine twice, copies two longs from global locations to offsets in two different output pointers, calls another routine twice more, and then spins until a global variable’s bit is cleared.
- `reset_dma_channels`: The routine performs a busy-wait loop, copies two values from one area of memory to another, adds a short offset to them, clears and initializes several word fields, calls another routine, then waits until a global variable no longer has a specific bit set.
- `populate_entity_state_2`: The routine iterates over a list of structures pointed to by a pointer in a register, increments a global counter when a flag is zero, and c.
- `populate_entity_state`: The routine iterates over a list of structures pointed to by a pointer in a register, increments a global counter when a flag is zero, and copies several clamped 16-bit values from parameters into the structure.
- `iterate_and_copy_clamped_values`: Iterates over a list of structures pointed to by a pointer in a register, increments a global counter when a flag is zero, and copies several clamped 16-bit values from another structure into the current entry.
- `increment_counter_and_copy_state`: Increments a 16-bit counter in a global variable area and copies several clamped 16-bit values from another structure into an output structure, also copying a 16-bit input value.
- `draw_palette_block`: The routine processes a palette block descriptor pointed to by its argument.
- `iterate_table_and_call_2`: The routine walks a table of pointers taken from a global variable, calling another routine for each non-zero entry and storing a zero through a pointer.
- `find_table_entry`: The routine clears a global word, selects a table pointer based on a parameter, scans up to 80 entries for a zero entry, writes the parameter into the first zero entry and sets a flag.
- `scan_table_and_call_palette`: Scans a table pointed to by a pointer until a zero entry is found, updates a global word and a pointer, and conditionally calls another routine with a value taken from a global variable.
- `initialize_and_wait_loop_2`: The routine stores a constant into a global word, copies a global dword, calls another routine three times, then runs a short countdown loop.
- `initialize_and_wait_loop`: The routine writes a constant to a global word, copies a global dword, calls another routine three times, then runs a short countdown loop calling another routine each iteration.
- `clear_flag_and_dispatch_2`: The routine clears a word in a global variable, calls another routine with two constants and then calls another routine with the result, the.
- `clear_flag_and_dispatch`: The routine clears a word in a global variable, calls another routine with two constants and then calls another routine with the result, then calls a function pointer stored in a global variable with a constant argument, and returns the value that was on the stack on entry.
- `process_tile_stream`: The routine repeatedly calls other routines in loops, obtains a length from a parameter, computes a count, and processes a byte stream by reading bytes and invoking a routine that writes masked bytes to three planes.
- `process_tile_stream_with_mask`: Processes a byte stream using a loop that reads bytes, updates a counter, and calls another routine that writes masked bytes to three planes.
- `modify_byte_and_spin`: Adds a byte from a parameter to a byte in a table indexed by a global variable, then calls another routine twice and enters an infinite spin loop by comparing a byte from a global variable with itself, finally returning a concatenated value from two input registers.
- `call_and_spin`: The routine calls another routine and then enters an infinite loop by comparing a byte from a global variable with itself, so execution never proceeds past the loop.
- `compute_sprite_tile_offsets`: The routine initializes several global variables from the sprite descriptor and a base pointer, then computes vertical and horizontal tile ranges against global limits and updates global offsets and bit masks accordingly.
- `process_sprite_collision`: The routine checks a vertical range against a global base and updates several global words with derived offsets and bit masks.
- `update_sprite_animation_and_position`: The routine updates several global words and dwords using values from a pointer and a global base, and repeatedly sets bits in a global word while calling another routine.
- `update_scroll_and_dispatch`: The routine repeatedly sets bits in a global word and updates several global words and dwords using values from a pointer and a global base.
- `find_table_entry_2`: The routine clears a global word, selects a table pointer based on a parameter, scans up to 80 entries for a zero entry, writes the paramete.
- `wait_for_vblank_and_copy_blitter_state`: The routine waits for a hardware flag to clear twice, then copies a series of values from a block of memory referenced via a base pointer into a global variable, repeatedly overwriting it.
- `adjust_scroll_and_redraw`: Moves the screen's scroll position back by the given amount, keeping it from going below zero, then redraws the screen edge.
- `update_entity_state_and_flags`: The routine clears two global words and then compares a combined value built from an input register and a global word with another combined value built from a second input register and a global word masked to clear low bits.
- `set_entity_state_and_update_position`: The routine initializes a small entity structure pointed to by a register, updates several global words that hold state flags and timers, and calls another routine to process the entity.
- `update_entity_state_flags`: The routine checks a global word flag and, if clear, stores a register value into a global word, updates several global words with a masked value, writes constants to a structure pointed to by a register, decrements a global word with underflow protection, and calls another routine.
- `update_entity_state_flags_2_2`: The routine updates a set of global variables that track an entity’s state and flags, writes a value from a register into a global word, and prepares a small structure pointed to by a register with several words before calling another routine.
- `update_entity_state_flags_2`: The routine checks a register bit and a global word flag, stores a register value into a global word, updates several global words with a masked value, writes constants to a structure pointed to by a register, and calls another routine.
- `update_position_and_flags_3_2`: The routine reads flag bits from global words, updates a structure pointed to by a register with several words, calls a routine that sets wo.
- `update_position_and_flags_3`: The routine reads flag bits from global words, updates a structure pointed to by a register with several words, calls a routine that sets words in a structure and computes a position, and updates additional global words based on flags.
- `update_position_and_flags_2`: Sets several words in a structure when a flag bit is set, then computes a 16-bit position from global flags and stores it, conditionally masking another global word, and copies a word from a global variable to another global location.
- `update_position_and_flags`: Computes a 16-bit position value from a flag in a global variable area and an input word, stores it, conditionally masks another word, and copies a word from a global variable area.
- `update_player_horizontal_position`: The routine updates a short horizontal position variable using a short velocity variable, clamps the position to a range, masks two flag words to the low two bits under certain conditions, and then derives a byte-sized position value by shifting the position right.
- `call_loop_with_return`: The routine invokes another routine, then repeatedly calls another routine with a value taken from a register and a previously returned value, followed by a call to another routine, looping while a word returned in a register is non-zero.
- `copy_scattered_bytes`: Copies a series of bytes from a source buffer addressed by an input word to a destination buffer addressed by a combination of global variables and input words, writing to multiple fixed offsets and then writing constant bytes.
- `copy_palette_entries`: The routine copies a series of bytes from a source byte table pointed to by a pointer to a destination byte table pointed to by another pointer, using a fixed set of offsets.
- `store_long_from_param`: Stores a 32-bit parameter into a 32-bit word at a fixed offset in a global variable area.
- `copy_palette_bytes_to_vram`: The routine computes a source pointer from a byte field of the first argument and a base table, computes a destination offset from a short field of the first argument, a scaled value from the second argument, and two global values, calls another routine, then copies eight bytes from the source to eight fixed offsets in the destination area.
- `cia_timer_interrupt`: CIA-A timer A interrupt routine (about 48 times a second): runs the sound and music step of the game.
- `vblank_interrupt`: Vertical-blank interrupt server: counts frames, runs the registered per-frame tasks, swaps the displayed screen buffer when a new one is ready and updates the copper list.
- `swap_pairs_and_clear`: If a 16-bit value in a global variable area is non-negative, swaps three pairs of 32-bit words at fixed offsets in the global variable area, copies a 32-bit value to a global variable, then clears that global variable.
- `swap_three_pairs_and_return`: The routine swaps the contents of three pairs of long words located at fixed offsets from a base pointer, then copies the value from the second swapped pair into a global variable.
- `fill_palette_entries`: The routine writes four 16-bit palette entries into two tables using a base pointer and an offset derived from global variables, computing each entry from a global integer and a short value with a constant added.
- `wait_for_vblank_and_clear_flags`: The routine waits until a short flag in a global structure becomes non-zero, optionally invoking a callback from a table if a global variable bit is clear and a local flag is not set, then invokes another callback from the same table, clears two word flags in the structure, and calls another routine twice.
- `decrement_until_negative`: The routine decrements a byte field in the argument and then repeatedly decrements it while it remains non-negative, using a loop that contains an empty spin on a byte from a global variable.
- `copy_word_table`: The routine copies 32 words from a source pointer to three destinations: a global table, another global table, and a third global table, using an index derived from a global variable.
- `store_indexed_table_entries`: It then calls another routine with one of the table base addresses and returns the concatenation of the two input registers.
- `clear_and_copy_word_table`: The routine clears a global flag, then repeatedly calls another routine and spins until a global byte stops changing, repeating this twenty-one times.
- `clear_flag_and_poll_loop`: The routine checks a global flag and, if set, clears it and then repeatedly calls another routine while spinning on a byte comparison until a counter expires.
- `clear_word_table`: The routine clears a block of 32 words in a global table by writing zero to each entry, then calls another routine with the table base address.
- `blend_colour_component`: Inner step of the palette fader: scales one four-bit colour component by the fade level and puts it back into the colour value.
- `copy_word_block`: The routine copies a fixed number of 16-bit words from a source pointer to a destination buffer located at a fixed offset from a global variable.
- `init_blitter_copper_lists`: The routine fills two parallel tables with pointers and word values, writes several constants into them, then calls another routine twice and updates a global variable.
- `fill_fixed_point_table`: The routine writes a series of fixed-point values into two tables using two input values and two base pointers.
- `keyboard_interrupt`: CIA-A serial port interrupt routine: takes the key code the keyboard sent, does the handshake and passes it to the key handler.
- `process_keyboard_input`: The routine reads the CIAA keyboard serial register, normalizes the character using a table and a global variable, stores the result in a global word, and then scans a character table to dispatch a handler based on the normalized character.
- `call_other_routine_2`: The routine calls another routine and returns the value left in a stack slot.
- `lookup_character_and_dispatch`: Normalizes an input character, stores it in a global word, then scans a character table comparing each entry with the input and dispatches to another routine with a table-derived argument when a match is found.
- `dispatch_character_lookup`: The routine loops over characters, normalizes an input character and stores it in a global word, then scans a character table comparing each entry with the input and dispatches to a routine.
- `store_byte_by_d2_selector_3`: The routine stores a byte from d0 into one of two global byte locations selected by the value in d2, using a selector routine, and then clears d2 before returning.
- `store_byte_by_d2_selector_2`: The routine selects one of two global byte locations based on the value in d2 and stores the byte from d0 into the chosen location.
- `store_byte_by_d2_selector`: The routine selects one of two global byte locations based on the value in d2 and stores the byte from d0 into the chosen location.
- `conditionally_store_byte`: Stores a byte from a register into a global byte location only when a data register holds a specific value.
- `clear_register_and_return`: The routine clears a data register and returns immediately.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d2, then returns.
- `incremental_table_store`: The routine checks a global counter stored in a global variable.
- `decrement_loop_table_call`: The routine loops while a short value stored at a fixed offset from a base pointer is non-zero, decrementing that short each iteration and calling a routine taken from a table indexed by the decremented value.
- `clear_word_via_frame_pointer`: The routine stores zero to a word located at a fixed offset from the frame pointer and returns.
- `add_vblank_task`: Adds a routine and its argument to the list of up to sixteen tasks that the vertical-blank interrupt runs every frame.
- `empty_loop_routine_2_2`: The routine contains nested loops that decrement two short counters to -1 without performing any observable work and then returns a 64-bit v.
- `clear_globals_and_return`: Calls two routines that clear memory via the frame pointer and then stores zero to a long at a fixed offset from a global base.
- `clear_long_via_frame_pointer`: The routine stores zero to a long located at a fixed offset from the frame pointer and returns.
- `read_input_and_fetch`: The routine loops calling routines that read joystick state and fetch a word with wrap index, checking returned values and exiting when a specific value is found, then returns the accumulated result.
- `empty_routine`: The routine contains four instructions and performs no observable work; it ends immediately.
- `read_joystick_and_update_state`: The routine reads a global variable that reflects the current input and button state, derives a directional mask from the joystick port and the fire buttons, and updates a short counter stored via an address in A4.
- `clear_two_words_via_frame_pointer`: The routine stores zero to two consecutive words at fixed offsets from the frame pointer and returns.
- `fetch_word_with_wrap_index`: Reads a word from a table using an index stored in a global variable, updates the index with wrap logic, stores the updated index back to a global variable, and returns a combined value.
- `fetch_word_and_update_index`: The routine reads a word from a table using an index, updates a short index with wrap logic, stores the updated index to a global variable, and returns the fetched word combined with a stack value.
- `store_value_in_circular_table`: The routine updates a circular index stored in a global variable and writes a 16-bit value from a register into a table of 16-bit entries indexed by that variable.
- `process_tile_stream_loop_2`: The routine initializes several global words and bytes and a local buffer, computes a character from a parameter and stores it in global bytes.
- `process_tile_stream_loop`: The routine compares a register byte with a global byte and calls another routine if they match, then enters a loop that increments a register byte with wrap logic, fetches a word via another routine, and conditionally calls other routines, writes bytes to a buffer, updates a global word, and invokes a code pointer from a global variable repeatedly until a condition is met.
- `conditional_blend_tile_2`: The routine checks two registers for inequality and calls a routine that blends a tile into three planes only when they differ.
- `tiny_move`: Small routine of 1 instruction that copies 63 to d2.
- `conditional_blend_tile`: Calls a routine that blends a tile into three planes only when two registers differ.
- `complex_tile_processing_loop`: Performs a loop that writes bytes to a buffer, calls a tile blending routine, and repeatedly invokes other routines based on values read from global variables and a returned word, with multiple nested loops and conditional branches.
- `complex_tile_processing_loop_2`: The routine performs a loop that writes bytes to a buffer, calls a tile blending routine conditionally, and repeatedly invokes other routines based on values read from global data, updating registers and buffer pointers.
- `self_recursive_stub`: The routine contains no instructions and immediately returns.
- `check_and_clear_flag`: The routine compares several words read through the frame pointer against fixed constants and sets a flag based on the match.
- `zero_pointer_and_return`: The routine stores zero through a pointer argument and returns a constant zero.
- `copy_cstring_and_call_handler`: The routine copies a null-terminated string from a pointer obtained via the frame pointer into a buffer at a fixed offset, then calls a handler obtained from a global variable and conditionally calls it again, referencing the text "ram:".
- `handle_input_and_update_state`: The routine clears a global word, computes a sum from two global words and stores it, sets a global word to a constant, calls a code pointer from a global variable, then calls another routine and conditionally updates several global words and calls additional code pointers from global variables before returning a combined value.
- `copy_cstring_and_call_handler_2`: The routine copies a null-terminated string from a pointer obtained via the frame pointer into a buffer at a fixed offset, then calls a hand.
- `accumulate_input_and_return_stack`: The routine adds an input value to a long at a fixed offset from the frame pointer and returns a value built from a stack location and a stack value.
- `initialize_huffman_decoder_state_2`: The routine initializes several decoder state variables from a table pointer, aligns a bit buffer, calls another routine to obtain a tree po.
- `initialize_huffman_decoder_state`: The routine initializes several decoder state variables from a table pointer, aligns a bit buffer, calls another routine to obtain a tree pointer, and then processes a Huffman tree using bit reading logic, calling huffman_read_tree.
- `huffman_read_tree`: Reads the Huffman code tree of a packed file recursively from the bit stream: a one bit makes an inner node with two subtrees, a zero bit a leaf whose eight-bit symbol follows, least significant bit first.
- `initialize_and_query_handler`: The routine writes constants to offsets from the frame pointer, calls two handlers obtained via the frame pointer, invokes another routine and conditionally updates a word from the frame pointer using a handler result, then calls two more handlers and returns a value built from the frame pointer.
- `invoke_library_handlers_and_reset_flag`: The routine stores a constant into a global variable, calls several library routines via pointers taken from a global variable, calls another routine, calls another library routine via a pointer, clears the global variable and returns a value from a register.
- `set_flag_and_call_handlers`: The routine writes a word to a global variable, calls a handler from a table, checks a result from another table entry and conditionally calls another handler, then calls a second handler from the first table and clears the word.
- `call_avail_mem`: Library stub: loads its arguments and the exec library base, calls AvailMem and returns its result to the caller.
- `invoke_global_handler`: The routine calls a function pointer stored at an offset from a global variable and returns.
- `set_flag_and_repeat_handler`: The routine writes a word to a global variable, calls a handler from a table, then repeatedly calls a handler from another table and checks a result up to three times, calls a second handler each time the result is non-zero, then calls a handler from the first table and clears the word.
- `set_flag_and_update_state`: The routine writes a word to a global variable, calls a handler from a table, checks a result from another table entry and if non-zero stores a value returned by another table entry into a global variable and calls several handlers from the same table, then calls a handler from the first table and clears the word.
- `shape_frame`: Returns the address of frame n of a loaded shape file, walking the frame headers (each frame holds four bitplanes and a mask of the size given in its header) and filling in the frame's image and mask pointers.
- `perform_shell_sort`: The routine performs an in-place shell sort on a word array pointed to by a register, using a gap that is halved each outer iteration and swapping elements between two interleaved regions.
- `sort_word_array_in_place`: The routine repeatedly swaps word elements between two interleaved regions of a table pointed to by a register, using several short counters from registers to control the loop and step size, and returns a value from the stack when the step size reaches zero.
- `return_stack_value`: The routine returns the 64-bit value passed on the stack without modification.
- `install_error_handler_and_forbid`: The routine calls a code pointer from a global variable, stores the result in a global word, and if zero returns after calling a library routine.
- `select_error_handler_and_invoke`: The routine builds a small structure on the stack with a constant and a handler pointer, chooses a data pointer based on a character returned by a library call, stores a handler address, the chosen data pointer and a constant into the structure, then calls a library routine with the structure and returns the structure contents.
- `setup_error_handler`: The routine selects a pointer based on a register value and stores a handler address, a data pointer, and a constant into a structure pointed to by a register, then invokes a handler.
- `display_error_and_exit`: The routine calls a handler twice with a fixed address and a value obtained from another routine, using the referenced text for a read/write error message, and returns a value built from a local variable and stack data.
- `empty_loop_routine_2`: The routine contains nested loops that decrement two short counters to -1 without performing any observable work and then returns a 64-bit value formed by concatenating two input registers.
- `parse_signed_decimal_string_3`: The routine skips leading spaces and tabs in a string, detects an optional sign, then accumulates a short value by multiplying by ten and adding digit values while a table lookup indicates a digit, and finally negates the result if a minus sign was seen.
- `parse_signed_decimal_string_2`: The routine examines the first byte of a string for a sign, skips an optional sign, then accumulates a short value by multiplying by ten and adding digit values while a table lookup indicates a digit, and negates the result if a minus sign was seen.
- `parse_signed_decimal`: The routine parses a signed decimal string pointed to by an unreferenced register, skipping an optional plus sign, accumulating digits using a table lookup, and negating the result if a flag register is non-zero.
- `parse_signed_decimal_string`: The routine converts a null-terminated decimal string pointed to by a register into a signed short by accumulating digits, using a table indexed from the frame pointer to test continuation, and negates the result if a sign flag is set.
- `empty_routine_6`: This routine performs no observable work and ends immediately.
- `generate_random_word`: The routine calls a function through a pointer stored in a global variable, adds a constant to the result and stores it in a global variable, then returns the high 15 bits of that stored value.
- `seed_random_state`: The routine stores the high 16 bits of its parameter into a global variable.
- `clear_buffer_and_initialize_io`: The routine obtains a value from another routine, clears a block of memory in a global variable by writing zeros in a loop, stores a register value and global data into global variables, conditionally calls a library routine, obtains another value via a library call, stores it, and then either calls a library routine or another routine before returning.
- `empty_routine_2`: The routine contains no instructions that perform work and returns immediately.
- `initialize_window_and_io`: The routine allocates and initializes a window structure, sets flags in it, stores a constant into a field, and calls several DOS and library wrappers for current directory, input, output, open, and library vectors.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d5.
- `parse_quoted_string_list`: The routine uses a global variable to locate a string table and computes an offset from a parameter and a byte from that table.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d0.
- `copy_bytes_with_terminator`: The routine copies bytes from a source pointer to a destination pointer, stopping early if a zero byte is encountered or a byte count expires, then writes zero bytes to fill any remaining space in the count, and returns the original destination pointer.
- `copy_string_with_limit_2`: The routine measures the length of the source string by scanning for a null terminator and then copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches its limit.
- `copy_string_with_limit`: The routine copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter in the third argument reaches -1.
- `copy_string_to_buffer`: The routine copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `multiply_32_bit_values`: The routine takes two 32-bit values from registers and returns a 64-bit product formed from the high and low 16-bit halves of each operand.
- `open_window_with_icon`: The routine stores the first parameter into a stack slot and calls another routine with the routine itself and the second parameter.
- `store_param_to_stack_slot`: The routine stores the first parameter into a stack-relative slot and then calls another routine with the routine itself, the second parameter and a stack address.
- `format_and_increment_counter`: The routine stores a parameter into a global dword, calls another routine with a pointer to a stack slot and a parameter, then writes a zero byte through a pointer derived from the global dword and returns a word result.
- `increment_and_store_byte`: The routine loads a pointer from a stack-relative slot, increments an integer stored at that slot, stores the low byte of the parameter through the loaded pointer, and returns that byte.
- `recovered_routine_analysis`: The routine receives a parameter and invokes another routine, passing itself as the first argument, the incoming parameter as the second argument, and a fixed stack address as the third argument, then returns.
- `call_and_return_concatenated_2`: The routine has 48 instructions, is called by one unnamed routine, and calls call_and_return_concatenated and recursive_call.
- `parse_format_string`: The routine walks a format string byte by byte, invoking a code pointer for each literal character and handling percent conversions by parsing flags, width, precision and length modifiers, then storing a small descriptor in a stack buffer and updating a character count.
- `sign_combine_and_negate`: The routine reads two signed inputs, flips a sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `sign_adjust_and_divide`: The routine calls another routine, adjusts the sign of a result based on the sign of an input register, and returns a 64-bit value formed by concatenating the adjusted value with an input register.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `perform_32bit_division`: The routine calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `self_calling_stub`: The routine calls another routine, passing itself as the first argument, the incoming parameter as the second argument, and a fixed stack address as the third argument.
- `empty_routine_5_2`: The routine contains no instructions and performs no operations.
- `empty_routine_5`: The routine contains no instructions and performs no operations.
- `store_byte_from_offset_2`: The routine reads a pointer from an address pointed to by an unreferenced register, increments that pointer, reads a byte from an offset in it, and calls another routine.
- `store_byte_from_offset`: The routine reads a pointer from an address pointed to by an unreferenced register, increments that pointer, reads a byte from an offset in another unreferenced register, stores the byte through the pointer, and returns the byte.
- `iterate_and_recurse`: The routine computes a start offset from a parameter and repeatedly calls another routine with successive offsets until a limit is reached.
- `clear_node_fields_2`: The routine checks a flag byte in a structure pointed to by a parameter and conditionally recurses on child pointers, combines returned words, and then clears the first four fields of the structure before returning the accumulated word.
- `clear_node_fields`: The routine tests a flag byte in a structure pointed to by a register and conditionally calls other routines, combines a returned word with an accumulator, and then clears several fields of the structure including words and a byte, returning the accumulator.
- `clear_node_and_update_fields_2`: The routine examines a short field in a structure and conditionally clears flags and zeroes fields, otherwise it updates fields in a structure.
- `clear_node_and_update_fields`: The routine examines a short field in a structure and conditionally clears flags and zeroes fields, otherwise it updates fields in a structure.
- `process_node_fields`: The routine examines a short field in a structure and conditionally clears flags and zeroes fields, otherwise it updates fields in a structure pointed to by a register, sets a flag byte, reads a byte from an offset in another structure and returns a value derived from that byte.
- `update_node_fields`: The routine updates fields in a structure pointed to by an unreferenced register, sets a flag byte, reads a byte from an offset in another structure, and returns a word derived from that byte or a sentinel value after calling helper routines.
- `update_pointer_and_store_byte`: The routine updates fields in a structure pointed to by an unreferenced register, sets a flag byte, reads a byte from an offset in another unreferenced register, stores the byte through a pointer derived from the structure, and returns a masked combination of a high word and the stored byte.
- `search_and_clear_entry`: The routine walks a structure list stored at a fixed offset from a register by advancing the pointer by a fixed stride until a flag byte is zero, then clears three words at that entry and returns the pointer.
- `search_and_clear_structure`: The routine iterates through a structure list by advancing a pointer by a fixed stride until a flag byte is zero, then clears three words in the structure and returns the pointer, or returns a null pointer if the end is reached.
- `clear_three_words`: The routine stores zero into three consecutive words at an address pointed to by an unreferenced register and then returns.
- `initialize_node_fields`: The routine calls another routine to obtain a value and writes constants and derived values into fields of a structure pointed to by a parameter, setting a word, a byte flag, and a pointer field, and conditionally sets an additional flag based on another routine's return.
- `set_flags_and_copy_data`: The routine writes a constant to an offset in an unreferenced register, sets a flag byte in that register, copies a word from another unreferenced register into the first, calls another routine and sets an error flag byte if the call returns non-zero.
- `traverse_linked_list_and_clear_2`: The routine loads a pointer from a global variable, walks the linked list by following the first word of each node, calls another routine with the node pointer and a computed offset for each node, and finally clears the global variable to zero.
- `traverse_linked_list_and_clear`: The routine walks a linked list pointed to by a register, calling another routine with each node and a computed offset until the pointer becomes null.
- `store_node_and_link`: The routine stores a self-reference into a stack slot, calls another routine to obtain a node pointer, writes a pointer from a global variable and a parameter into the node, updates a global link, and returns the next pointer.
- `store_node_and_return`: The routine copies a pointer from a global variable into a caller-supplied location, copies a word from another argument into the next slot, stores the caller-supplied pointer back into the global variable, and returns a pointer two words past the supplied location.
- `store_node_and_link_2`: The routine stores a self-reference into a stack slot and calls another routine to obtain a node pointer.
- `remove_list_node_and_recurse`: The routine walks a linked list obtained from an unreferenced register until it finds a node matching a parameter, relinks the list to remove the node, then calls another routine with the node and a derived value and returns zero.
- `traverse_list_and_store`: The routine walks a linked list using pointers from an unreferenced register until a matching entry is found.
- `store_pointer_and_exit`: The routine copies a pointer from one unreferenced register to another or to a global variable depending on a null check, calls another routine, and returns zero.
- `store_pointer_and_call`: The routine stores a word from a register into a global variable, calls another routine, and then calls a library stub that loads a library base from a global variable and invokes a function at a fixed offset, returning the result of that call.
- `validate_index_and_recurse`: The routine checks a parameter against a short stored in a global variable and a table entry obtained via a scaled index.
- `check_input_table_entry`: The routine reads a short from a global variable, scales it by six, adds it to a pointer stored in a global variable, dereferences the result to obtain a pointer, and calls another routine with the value at that pointer.
- `return_zero_immediately`: The routine contains no observable work and returns zero.
- `check_and_store_result`: The routine calls another routine with the input parameter and checks its return value.
- `empty_stub_routine`: The routine contains no instructions and performs no operations.
- `check_error_and_call_helpers`: The routine has nineteen instructions, is referenced by address and called by four callers, and calls two other routines that test a word for a mask and write a constant on failure and that call another routine with fixed arguments and test a bit in the result.
- `conditional_error_check`: The routine tests a word from a pointer for a mask and writes a constant to a global variable on failure.
- `check_and_store_error`: The routine calls another routine with arguments taken from a global variable and two offsets from another pointer, then if the result is -1 it calls another routine, stores the returned word into a global variable at an offset from a pointer, and returns -1; otherwise it returns the result.
- `return_immediately_2`: The routine contains no observable work and returns without a value.
- `check_library_vector_flag`: The routine calls another routine with fixed arguments, tests a bit in the result, and if set returns the result when a global short is zero or otherwise calls another routine.
- `call_routine_and_return`: The routine calls another routine and then returns zero.
- `call_and_jump_indirect`: The routine calls another routine with a global variable and a constant, then calls another routine with the result, and finally jumps to an address computed from an uninitialized register.
- `invoke_optional_handler`: The routine checks a global pointer and calls the indirect code it points to if non-zero, then calls another routine unconditionally.
- `process_global_flags_and_tables_2`: The routine checks several pointers stored in an unreferenced register and calls indirect code or other routines with values derived from global variables, loops a counter up to a limit read from a global variable, and returns a word loaded from a computed pointer.
- `process_global_flags_and_tables`: The routine increments a counter up to a limit read from a global variable and calls helper routines with values derived from global variables.
- `conditional_library_calls`: The routine tests two flags stored in a global variable and conditionally calls library functions via vectors, passing values loaded from the same global variable, and finally returns a word loaded from another argument.
- `call_library_functions_and_return`: The routine calls a library function via a vector, then calls another library function via a vector with a value loaded from a stack-relative location, and returns a value loaded from a different stack-relative location.
- `empty_routine_3`: The routine contains three instructions and performs no observable work; it ends immediately.
- `clear_table_entry_conditionally`: The routine computes a pointer into a table using a global base and an index from the parameter, validates the index against bounds and a table entry, stores an error code into a global variable and returns a constant on failure, otherwise optionally calls another routine and clears the table entry before returning zero.
- `conditional_close_and_clear`: The routine tests a flag byte in a caller-supplied structure and conditionally calls another routine with the structure pointer, clears the first word of the structure to zero, and returns zero.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_414_2`: The routine loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_198_2`: The routine loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552_2`: The routine loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_minus_78`: The routine loads a library base from a global variable and calls the library function at offset minus 78 of that base.
- `call_library_vector_426`: Library stub: loads a library base from a global variable and calls the library function at offset minus 426 of that base, returning its result to the caller.
- `allocate_memory_and_build_mask`: The routine stores parts of its parameter into an unreferenced register, calls a helper obtained from a global variable, and on success stores the result and a pointer into the unreferenced register, updates a global variable, and returns a combined value built from the allocation result and an input register.
- `parse_music_event_header`: The routine reads a music event header pointed to by a parameter, clears several global variables, and sets up a table of pointers to event data blocks.
- `clear_audio_channels`: If a global flag is set it clears it, then iterates over three audio channels writing zero to AUD3VOL via a pointer that steps through the custom chip address space and toggles DMACON with bit 15 set.
- `store_negated_parameter`: Stores the negated low word of the parameter into a global variable at a fixed offset and returns the incoming register unchanged.
- `conditional_audio_clear`: The routine initializes a return value to zero and, if a global flag is non-zero, calls another routine that may clear audio channels, then returns the obtained value.
- `process_dma_timer_tick`: The routine is called from the CIA timer interrupt and updates a set of global words and shorts that form a DMA timing structure.
- `manage_dma_interrupts_2`: The routine increments a counter in a structure and, when a condition is met, clears several words, indexes into tables using values from the structure to load a short, updates counters, runs a fixed-count loop calling another routine, performs a second fixed-count delay loop, combines a word from the structure with a global variable using bit operations, and conditionally resets fields in the structure.
- `manage_dma_interrupts`: The routine manipulates several global words and shorts, performs loops with fixed iteration counts, updates a global variable with bit manipulations, and touches DMACON and INTENA/INTREQ hardware registers to control DMA and interrupts.
- `select_and_update_entity`: The routine selects an entity entry from a table using an index derived from a flag and a global counter, copies fields from the entry into a working structure, merges a word into a global variable, and updates several global variables and a destination structure with values from the entry.
- `copy_fields_and_update_globals`: Copies several words and bytes from a source structure pointed to by a parameter into a destination structure, merges a word into a global variable with an OR, writes a shifted value to another global, sets two words in another structure, masks a combined value and writes it to a global, then conditionally copies and compares fields from the destination structure using values from global variables.
- `compute_and_store_delta_2`: Selects a word from a structure based on a flag, subtracts a value from a global variable, clamps the result to zero if negative or if another flag is set, and writes the result to a short field.
- `compute_and_store_delta`: Selects a word from a structure based on a flag, subtracts a value from a global variable, clamps the result to zero if negative or if another global flag is set, stores the result into a structure field and returns a constant minus a register value.
- `build_interrupt_mask_loop`: The routine calls an empty helper and builds a mask from a global variable and an input register.
- `set_interrupt_mask_and_copy`: The routine builds a bit mask from an input register, stores it into a global variable and copies words from a structure pointed to by an argument into a local buffer.
- `return_stack_value_2_2`: Stores a byte zero into a byte at an offset from a pointer and copies the low word of a register into a global variable.
- `return_stack_value_2`: Returns the 64-bit value passed on the stack without modification.
- `scan_audio_structures`: The routine iterates over a fixed number of structures referenced via a global base, advancing a pointer each iteration and calling another routine while a condition involving a global byte and a global word holds, exiting early when the counter expires.
- `process_audio_event`: The routine reads bytes from a structure pointed to by a register and from several global variables, selects values based on flags and nibbles, calls other routines to adjust audio-related state, and returns a combined 64-bit value built from parts of its inputs and the structure.
- `lookup_nibble_scaled_value`: The routine selects a nibble from a byte in a structure based on a masked index and uses it to index a table of shorts.
- `lookup_scale_value_high_nibble`: The routine extracts the high nibble of a byte from a structure, uses it as an index into a table of shorts and searches the table for a matching word from the structure.
- `lookup_scale_value_low_nibble`: The routine extracts the low nibble of a byte from a structure, uses it as an index into a table of shorts and searches the table for a matching word from the structure.
- `store_register_to_memory_2`: Copies a word from an offset of one pointer to an offset of another pointer and returns.
- `scale_register_value_2`: Searches a table of shorts for a match with a word from a pointer, adjusts an index derived from an input register, and stores the selected short into a word at an offset from a pointer.
- `store_register_to_memory`: Stores the low word of a register into a word at an offset from a pointer.
- `scale_register_value_3`: If a short from a global offset is less than a threshold, it calls another routine, adds a masked byte from a pointer to a value in A0, clamps the sum to a maximum, and stores the result into a word at an offset from a pointer.
- `adjust_output_with_threshold`: The routine checks a short from a global offset against a threshold and, if the check fails, calls another routine and adjusts a word pointed to by A0 using a masked byte from a structure.
- `clear_audio_channels_2`: Reads a byte from an offset of a pointer, substitutes a default if zero, and either calls another routine or masks the value and stores it into a global variable at a fixed offset.
- `mask_and_store_word`: Masks the low word of the input register with 65295 and stores the result into a global variable at a fixed offset.
- `clear_global_bit_2`: Tests a bit in a byte from a pointer, clears a bit in a global variable or sets another bit, and returns the byte shifted right by one.
- `clear_global_bit`: Clears a bit in a global variable by ANDing it with 253 and returns.
- `scale_register_value`: Returns the product of four and the difference between three and a register value.
- `process_effect_script`: If a global flag is set and a pointer matches a marker, it searches for sub-markers in the pointed data, writes derived values into a table entry computed from a parameter and a global base, sets a byte flag and a color-related word, then falls through to a loop that repeatedly writes 4095 to a global variable before returning the incoming registers.
- `process_effect_script_2`: If a global short is zero it clears a global variable repeatedly in a loop and returns the concatenation of two input registers; otherwise it writes a zero byte at a computed address derived from a parameter and a global pointer and returns the concatenated registers.
- `process_audio_channel_and_delay`: The routine conditionally copies fields from a table entry into a structure pointed to by A6 and writes values to AUD0LCH, DMACON, INTENA and INTREQ hardware registers.

### Castlevania (1990)(Konami)[cr QTX][a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `infinite_loop`: The routine contains a single instruction that forms an endless loop and performs no other operations.
- `infinite_loop_stub`: The routine contains no instructions that modify data or call other code; it is an empty infinite loop that never returns.
- `infinite_loop_stub_2`: The routine contains an empty loop that never exits.
- `infinite_loop_routine`: The routine contains only an unconditional branch that repeats forever, performing no operations on data, registers, or memory.
- `infinite_loop_2`: The routine contains no instructions and consists of an empty infinite loop.
- `infinite_loop_routine_2`: The routine contains no instructions and immediately enters an endless loop, performing no operations on data.
- `infinite_loop_stub_3`: The routine contains only an unconditional branch that keeps the program counter on the same instruction, so it never reaches any other code and performs no observable work on memory or registers.
- `infinite_loop_stub_4`: The routine contains only an unconditional infinite loop and performs no operations on memory or registers.
- `infinite_loop_stub_5`: The routine contains no instructions and immediately enters an infinite loop, performing no operations on data or registers.
- `infinite_loop_stub_6`: The routine contains no instructions and immediately enters an infinite loop, performing no operations on data.
- `infinite_loop_3`: The routine contains a single instruction that forms an endless loop with no operations.
- `infinite_loop_stub_7`: The routine contains no instructions and immediately enters an infinite loop.
- `infinite_loop_stub_8`: The routine contains no instructions and is an empty stub.
- `infinite_loop_routine_3`: The routine contains no instructions and consists of an empty loop that never exits.
- `infinite_loop_stub_9`: The routine contains only an unconditional branch that repeats forever, so it performs no observable work and never returns.
- `infinite_loop_routine_4`: The routine contains no instructions and immediately enters an infinite loop.
- `infinite_loop_stub_10`: The routine contains no executable operations and immediately enters an endless loop, performing no work on data.
- `infinite_loop_routine_5`: The routine contains an empty loop that never terminates.
- `infinite_loop_routine_6`: The routine contains no instructions and consists of an empty infinite loop that never exits.
- `infinite_loop_stub_11`: The routine contains no instructions and consists of an empty infinite loop.
- `infinite_loop_4`: The routine contains no instructions and forms an infinite loop.
- `infinite_loop_5`: The routine contains no instructions and forms an infinite loop.
- `infinite_loop_stub_12`: The routine contains no instructions and therefore performs no operation.
- `infinite_loop_stub_13`: The routine contains no instructions and consists of an empty infinite loop.
- `infinite_loop_stub_14`: The routine contains no instructions and consists of an empty infinite loop that never exits.
- `infinite_loop_6`: The routine contains no executable instructions and consists of an empty infinite loop that never exits.
- `infinite_loop_stub_15`: The routine contains no instructions and consists of an empty infinite loop.
- `loop_empty`: The routine contains no instructions and therefore performs no operation.
- `infinite_loop_stub_16`: The routine contains no instructions and consists of an empty infinite loop.
- `infinite_loop_stub_17`: The routine contains no instructions and consists of an empty infinite loop.
- `endless_loop_routine`: The routine contains no instructions and immediately enters an infinite loop, performing no operations on data or registers.
- `infinite_loop_stub_18`: The routine contains no instructions and consists of an empty infinite loop.
- `infinite_loop_routine_7`: The routine contains no instructions and consists of an empty loop that never terminates.
- `infinite_loop_routine_8`: The routine contains no instructions and is an empty function.
- `infinite_loop_stub_19`: The routine contains no instructions and consists of an empty infinite loop.
- `infinite_loop_routine_9`: The routine contains no instructions and consists only of an infinite loop that never exits.
- `infinite_loop_stub_20`: The routine contains no instructions and consists of an empty infinite loop.
- `infinite_loop_stub_21`: The routine is empty except for an unconditional infinite loop.
- `infinite_loop_routine_10`: The routine contains no executable instructions and consists of an infinite loop that never exits.
- `infinite_loop_stub_22`: The routine contains no instructions and immediately enters an infinite loop, performing no operations on memory or registers.
- `empty_loop_routine`: The routine contains an infinite loop with no operations.
- `infinite_loop_routine_11`: The routine contains an empty loop that repeats forever, performing no operations and never returning.
- `infinite_loop_routine_12`: The routine contains no instructions and is an empty infinite loop.
- `infinite_loop_routine_13`: The routine contains an empty loop that repeats forever.
- `infinite_loop_stub_23`: The routine contains no instructions and ends with an unconditional branch to itself, forming an infinite loop.
- `infinite_loop_stub_24`: The routine contains no instructions that modify memory or registers; it consists of an empty loop that never exits.
- `infinite_loop_7`: The routine contains no instructions and consists of an infinite loop that never exits.
- `infinite_loop_routine_14`: The routine contains no instructions and therefore performs no operation.
- `infinite_loop_8`: The routine contains no instructions and consists of an empty infinite loop.
- `infinite_loop_stub_25`: The routine contains no instructions and consists of an empty infinite loop.
