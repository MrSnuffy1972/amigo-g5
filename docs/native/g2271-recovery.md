# Game 2271 (castlevania (1990)(konami)[cr qtx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02271). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901664 top-level bytes in 127 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 594 of 594 named (0 author symbols), 594 of 594 described (88 by rule, 459 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 10031 of 10031 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 20202 of 20202 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 287843 |
| code | 67146 |
| data | 485185 |
| padding | 29234 |
| unused | 32256 |

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
| Castlevania (1990)(Konami)[cr QTX].adf:cast | 3 | 17918 | 416 |
| Castlevania (1990)(Konami)[cr QTX].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 77 | 77 |

## Cases handled

- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:art/BONE: packed data by evidence (same Huffman format (cast +D56C/+D622), checksum verified)
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:art/magic: high-entropy data file (7.63 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:art/ROBOKNIG: packed data by evidence (same Huffman format (cast +D56C/+D622), checksum verified)
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:art/DEMON: high-entropy data file (7.83 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:art/frog: high-entropy data file (7.79 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:art/FRANKIE: high-entropy data file (7.74 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:art/DEATH: high-entropy data file (7.79 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:art/goodies: high-entropy data file (7.64 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:art/SCAST: high-entropy data file (7.62 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:art/dracula: high-entropy data file (7.77 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:art/GHOUL: high-entropy data file (7.84 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:art/DOG: high-entropy data file (7.82 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:art/MUMMY: high-entropy data file (7.87 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:art/BAMMER: packed data by evidence (Huffman stream depacked by the game's loader (cast +D418 -> +D56C decode, +D622 tree): header word = 16-bit sum of the 614 output bytes, verified; all 36 art and 38 screens files decode the same way to the last byte)
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:art/CASTSHPS: high-entropy data file (7.90 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/VAN11: high-entropy data file (7.73 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/van12: high-entropy data file (7.84 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/VAN13: high-entropy data file (7.92 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/VAN14: high-entropy data file (7.90 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/van15: high-entropy data file (7.90 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/van16: high-entropy data file (7.92 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/VAN17: high-entropy data file (7.83 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/VAN18: high-entropy data file (7.91 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/van19: high-entropy data file (7.91 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/van20: high-entropy data file (7.93 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/van21: high-entropy data file (7.89 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/VAN22: high-entropy data file (7.91 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/VAN23: high-entropy data file (7.91 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/VAN24: high-entropy data file (7.94 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/VAN0: high-entropy data file (7.88 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/VAN25: high-entropy data file (7.92 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/VAN1: high-entropy data file (7.94 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/VAN26: high-entropy data file (7.90 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/van2: high-entropy data file (7.90 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/VAN27: high-entropy data file (7.92 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/VAN3: high-entropy data file (7.90 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/van28: high-entropy data file (7.86 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/van29: high-entropy data file (7.93 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/VAN4: high-entropy data file (7.92 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/VAN5: high-entropy data file (7.82 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/van6: high-entropy data file (7.81 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/van7: high-entropy data file (7.89 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/van8: high-entropy data file (7.90 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/VAN30: high-entropy data file (7.91 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/VAN31: high-entropy data file (7.92 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/van9: high-entropy data file (7.82 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/VAN32: high-entropy data file (7.92 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/VAN33: high-entropy data file (7.92 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/VAN34: high-entropy data file (7.90 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/van35: high-entropy data file (7.92 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/VAN36: high-entropy data file (7.93 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/van37: high-entropy data file (7.88 bits/byte), classified as packed data
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:screens/VAN10: high-entropy data file (7.89 bits/byte), classified as packed data
- runtime segment an address: 66 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:cast +765E..7664: data by evidence (printf format "%07ld" pushed with pea by the score printer; never executed); 1 instructions decoded there were removed
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:cast +A8E2..A8EC: data by evidence (two blank strings pushed with pea by the password screen; never executed); 4 instructions decoded there were removed
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:cast +A8EC..A8F4: data by evidence (printf format "%c %02d" pushed with pea by the password screen; never executed); 2 instructions decoded there were removed
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:cast +B00C..B028: data by evidence (menu title text pushed with pea by the in-game options menu; never executed); 13 instructions decoded there were removed
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:cast +DDB8..DDBA: data by evidence (one-character string pushed with pea; never executed); 1 instructions decoded there were removed
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:cast +DF6A..DF6C: data by evidence (one-character string pushed with pea; never executed); 1 instructions decoded there were removed
- castlevania (1990)(konami)[cr qtx]/Castlevania (1990)(Konami)[cr QTX].adf:cast +E09E..E0A0: data by evidence (one-character string pushed with pea; never executed); 1 instructions decoded there were removed

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### type

- `initialize_game_state`: The routine walks a linked list of entries to find the maximum of a field, then builds a small table in memory and copies a block of sixteen words between a global buffer and a stack area using a helper routine.
- `copy_table_to_stack`: The routine copies a fixed number of words from a table located at an offset from a pointer into a caller-supplied buffer on the stack, then calls another routine.

### Dir

- `initialize_memory_table`: The routine walks a linked list of entries in a global variable to find the maximum value stored at an offset from each entry, then writes that maximum into a table and fills the table with a sequential pattern.
- `copy_16_words_from_table`: The routine copies sixteen 32-bit words from a table located at an offset from a pointer in a2 into the memory pointed to by a pointer on the stack, then calls another routine.

### Echo

- `initialize_echo_buffer`: The routine walks a linked list of entries to find the largest size, then allocates a temporary buffer on the stack and fills a table with sequential values.
- `copy_stack_block`: The routine copies a fixed number of words from a table located at an offset from a pointer in a2 into a buffer pointed to by a stack argument, then calls another routine.

### Execute

- `initialize_memory_table_2`: The routine walks a linked list of entries in a global variable to find the maximum value stored at an offset, then writes that maximum into a table and fills the table with increasing word values.
- `copy_table_to_stack_2`: The routine copies sixteen 32-bit values from a table located at an offset from a pointer into a caller-provided buffer on the stack, then calls another routine.

### Copy

- `initialize_palette_table`: The routine computes the maximum size of a linked list stored in a global variable, then builds a palette table in RAM, fills it with a repeating pattern, sets up a small stack frame, copies sixteen words between two buffers via another routine, and returns a status code.
- `copy_16_words_from_table_2`: The routine copies sixteen 32-bit words from a fixed table located at an offset from a register into a caller-supplied buffer, then calls another routine.

### Makedir

- `initialize_directory_structure`: The routine walks a linked list of directory entries to find the maximum name length, then builds a small in-memory directory table with sequential word values and a header.
- `copy_table_to_stack_3`: The routine copies sixteen 32-bit values from a table located at an offset from a pointer into a caller-supplied buffer on the stack, then calls another routine.

### wait

- `initialize_memory_table_3`: The routine walks a linked list of entries to find the maximum value stored in a field of each entry, then builds a small table in memory with a header and a sequence of increasing words.
- `copy_table_to_stack_4`: The routine copies sixteen words from a table located at an offset from a pointer in A2 to a pointer passed on the stack, then calls another routine.

### disk-validator

- `initialize_disk_validator_state`: The routine writes a character derived from a global variable into a table via pointers, updates several fields in a global structure, clears a buffer, and then repeatedly calls another routine while setting up loop counters and bit masks in memory.

### ram-handler

- `call_indirect_table_entry`: The routine calls a function obtained by indexing a global variable and then calls another routine through an indirect jump table.

### serial.device

- `return_negative_one`: The routine returns a constant value with all bits set, which corresponds to -1 in signed arithmetic.
- `initialize_serial_device_state`: The routine clears several fields in a device structure pointed to by a register, copies a value from a register into a field and into multiple other fields, and writes shifted versions of that value into two further fields.
- `check_flag_and_call_handler`: The routine tests a byte at an offset from a pointer in A1 against a mask and, if the test passes, calls a handler routine obtained from a table via A6.
- `set_status_byte_and_call_handler`: The routine writes a constant byte to an offset in a structure pointed to by a1 and tests a flag byte at a nearby offset.
- `disable_interrupts_and_call`: The routine sets a flag in a byte at an offset from a register, writes a value to a global variable, calls another routine, writes a different value to the same global variable, and clears the flag in the byte.
- `remove_serial_message`: The routine clears a flag byte in the message structure and, depending on the message type, either removes the message from a linked list and updates a counter, or clears a pointer in a global structure and may invoke another routine.
- `clear_serial_buffer_entry`: The routine checks a pointer in a global structure against an argument and clears a flag byte and a pointer field if they match.
- `clear_serial_message`: The routine checks a flag byte in a structure pointed to by a1.
- `process_serial_message`: The routine processes a pointer passed in a register and a structure pointed to by a global variable.
- `decrement_counter_and_clear_flag`: The routine checks a flag byte in the structure pointed to by a1 and, if set, copies the structure’s first word through a pointer stored in the structure, updates a pointer stored at an offset from that word, decrements a short counter in a6, modifies two flag bytes in the structure and clears a byte.
- `call_indirect_via_table`: The routine calls another routine through an indirect pointer obtained from a global variable table.
- `check_and_update_link`: The routine checks a flag in a structure and, if clear, calls two routines via pointers stored in the structure, updates a link between two pointers in the structure, and returns a value from the structure.
- `clear_serial_device_state`: The routine invokes a series of callbacks stored in tables referenced by a global variable and another global variable, then clears a device state by writing a pointer from a global variable into a location pointed to by a second global variable and updating a linked pointer, and returns a value from a global variable.
- `handle_serial_interrupt`: The routine processes a serial device interrupt message.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `register_serial_device`: The routine links a serial device structure into a global device list and updates a device counter.
- `handle_serial_input`: The routine processes a serial device control block pointed to by A1, updating a global status variable based on flags in the block and a linked list of pending requests.
- `decrement_counter_and_branch`: The routine decrements a short counter stored in a structure and, if the counter reaches zero, writes a byte and a word from the structure to a buffer, clears a flag byte in the structure, zeroes another field, and calls another routine via a function pointer.
- `call_other_routine`: The routine calls another routine and then returns.
- `update_serial_device_state`: The routine updates fields in a device structure, clears and sets flag bits in a status byte, combines a mask from a table with a bit derived from another byte, increments two counters, optionally searches a small table for a matching value, and then calls another routine via a function pointer stored in the structure.
- `process_interrupt_request`: The routine checks a counter against a stored limit and updates several fields in a structure pointed to by A6, including a bit mask derived from a byte field and a value in D0.
- `update_serial_status_flags`: The routine merges a value returned by another routine with a flag from a control byte and a stored word, writes the result to a global variable and two counters, and conditionally searches a small table for a match.
- `decrement_counter_and_update_linked_list`: The routine checks a counter in a structure and, if non-zero, reads an index from a field, updates a linked list pointer and the back link, stores the pointer, decrements the counter, toggles two bits in a byte of the referenced object, clears a field, copies a field from the object into the structure, and recurses if a field equals a constant.
- `set_global_flag`: The routine writes a constant value to a global variable and returns.
- `check_serial_interrupt_status`: Reads a byte from a global variable and combines it with a value from a hardware register to update a global status flag.
- `set_serial_device_state`: The routine writes to a global variable and to a structure pointed to by a register, clearing a byte field and zeroing several fields, then writes constant values into the structure.
- `check_serial_status_and_dispatch`: The routine calls another routine and then tests a byte from a structure pointed to by a register.
- `copy_serial_state`: The routine copies a 32-bit value from an offset within a structure pointed to by a register to two other offsets in the same structure, then clears a 32-bit field at a third offset.
- `handle_serial_device`: The routine checks a global variable and sets a flag byte in a structure pointed to by a6.
- `disable_serial_interrupt_handler`: The routine clears a flag byte in a structure, optionally calls another routine and stores its result, then writes a value to a global variable twice and writes to INTENA to enable interrupts.
- `disable_interrupts_and_call_handler`: The routine writes to a global variable, calls another routine, writes the global variable again, and conditionally calls a handler via a pointer taken from a table.
- `clear_serial_flag`: The routine calls another routine twice and then clears a 32-bit field in a structure pointed to by an address register.
- `process_serial_message_queue_3`: Iterates through a linked list of message structures pointed to by a register.
- `empty_routine`: The routine is empty.
- `set_serial_flags_and_maybe_call`: The routine sets a flag byte in a structure pointed to by a1 and writes a constant to the next byte.
- `copy_serial_state_flags`: The routine copies a byte from a global variable, masks it with a flag from a structure, writes the result to a structure field, copies and masks another byte from a structure, clears a flag in the structure, copies a word from the structure, and conditionally calls another routine via a function pointer stored in the structure.
- `clear_serial_input_flag`: The routine clears a flag byte in a message structure and, if a condition on the message and a global block is true, links the message into a queue held in a global block and updates counters and a global variable.
- `set_flag_and_call_routine`: The routine sets a bit in a byte located via a pointer, calls another routine, stores a value returned in A1 into a word at an offset from the same pointer, and writes a constant to a global variable.
- `clear_serial_flag_and_disable_interrupts`: The routine clears a flag in a byte field of a structure pointed to by a1, and if a condition on that byte and a short field is met it sets another flag in the same byte, clears a pointer field, writes a value to a global variable, and calls another routine via a pointer stored in the structure.
- `set_serial_timeout`: The routine sets a global flag and, when a byte in a structure is not a specific value, clears a field, clamps an input value to a minimum, writes a timeout word derived from the input, sets a command byte and a status byte, then calls another routine via a function pointer stored in the structure.
- `empty_routine_2`: The routine contains no instructions and performs no operations.
- `handle_serial_message`: The routine processes a byte from a message buffer pointed to by a register, updating a status byte in a structure pointed to by another register and clearing or setting bits in related fields.
- `clear_flag_and_dispatch`: The routine clears a flag byte in a structure and, if several conditions on bytes and words in that structure and in a pointed-to block are met, it sets a global variable, calls another routine, restores the global variable and copies a value from an output parameter into an input pointer.
- `disable_interrupts_and_call_handler_2`: The routine writes to a global variable, calls another routine, writes to the same global variable again, then conditionally calls a handler obtained from a table via an indirect pointer.
- `process_serial_message_2`: The routine validates a message structure in memory and, if the checks pass, copies several bytes and words from the input structure into a global variable, updates flag bytes in both structures, calls another routine twice, and stores results from the second call into the global variable.
- `compute_serial_status_word`: The routine builds a 16-bit status word from a byte in a structure and a flag in another structure, then stores the word into a global variable.
- `decrement_indexed_table_lookup`: The routine checks a flag byte in a structure pointed to by a register and conditionally decrements a short index.
- `set_serial_control_byte`: The routine writes a constant byte value to an offset within a memory block pointed to by a pointer.
- `compute_timer_interval`: The routine computes a 16-bit interval from a scaled value stored in the structure and a base constant, optionally adjusted by a flag byte, and writes the result to a field in the structure and to a global variable.
- `compute_serial_divisor_and_flags`: The routine divides an input value by the low word of another input and stores the result in a structure field, copies that field to a global variable, clears a flag bit in a byte field, and conditionally sets a high bit in the global variable and a flag bit in the byte field based on two structure bytes.
- `initialize_serial_device_unit`: Allocates and initializes a device unit structure by calling library functions to set up memory and resource references.
- `initialize_serial_device`: The routine initializes a serial device structure by calling several other routines, writing constant values into specific fields of the structure, setting up function pointers within the structure, storing return values from other routines into the structure, and setting a global variable before returning a success code.
- `initialize_serial_device_2`: The routine stores a pointer to a global variable in a structure pointed to by a register, writes a constant byte into the structure, and copies two caller-supplied values into fields of the structure.
- `set_serial_device_io_request`: The routine writes a constant byte to an offset in a structure pointed to by a0, stores a pointer to a global variable at another offset, and copies two caller-provided values into two further offsets.
- `set_serial_device_request`: The routine fills a request block in memory with a pointer to a global variable and several byte fields, then calls a function through a pointer obtained from a table.
- `initialize_serial_device_3`: The routine writes a byte to two offsets in a structure, sets two integer fields to pointers inside the same structure, calls a routine via a pointer in the structure, stores a routine pointer into two offsets, calls another routine twice and stores the returned values into two offsets, and sets a global variable.
- `set_serial_device`: The routine stores a value from a register into a structure at an offset, stores the address of a global variable into the structure, stores a constant byte into the structure, and then calls another routine via a pointer obtained from a table.
- `return_zero`: The routine is empty and returns zero.
- `register_serial_handler`: The routine clears a flag in a structure pointed to by a1 and, if a condition on a global variable is met, updates a linked list of handlers stored in a global variable and increments a counter, then calls another routine via a function pointer.
- `pop_serial_buffer`: The routine processes a serial device control block pointed to by a1 by repeatedly selecting a pending message from a linked list and advancing a circular buffer of bytes.
- `process_serial_queue`: The routine repeatedly decrements a counter stored in a global variable and advances pointers through a table of bytes, copying a byte from an input buffer to a global byte location.
- `decrement_output_counter_loop`: The routine clears a flag and updates status bytes, then calls another routine while a short counter is non-zero.
- `process_serial_message_queue`: The routine processes a serial message queue stored in a global variable.
- `process_serial_input`: The routine processes a serial input byte from a register, updates counters and flags in a global structure, and repeatedly dequeues entries from a linked list while calling another routine for further handling.
- `do_nothing_stub`: The routine contains no instructions and returns immediately.
- `identify_routine`: This routine is not empty; it contains ten instructions but the decompiler has not produced meaningful pseudo-code.
- `process_serial_message_queue_2`: The routine clears and updates several flags in a global structure and in the message structure pointed to by A1, sets a status byte in the message, and then processes a queue of messages by repeatedly dequeuing an entry, clearing its flags, invoking a callback from a function pointer table, and updating pointers and counters until the queue is empty or a stop condition is met.
- `set_request_error_from_status`: Part of the serial device's request completion: sets the error field of the I/O request from the device's status flags (break or line error), clears those flags and finishes the request.
- `process_serial_ring_buffer`: The routine manages a serial device ring buffer and a per-message state structure.
- `process_serial_message_3`: The routine examines a word from the first argument and a set of flags in the second argument to decide whether to set a global variable, update a buffer pointer and length fields in the second argument, and invoke a callback stored in the second argument.
- `disable_interrupts_and_process_buffer`: The routine disables interrupts by writing to INTENA and then processes a byte buffer pointed to by a structure in A6, copying bytes from D0 into the buffer and updating length and pointer fields.
- `process_serial_buffer`: The routine processes a byte buffer referenced by a structure pointer.
- `store_byte_and_advance_pointer`: The routine stores a byte from a register into a buffer via a pointer, advances the pointer and a counter, and wraps the pointer when it reaches a limit.
- `handle_serial_byte`: The routine processes a byte from a register using a pointer stored in a structure.
- `set_flag_and_call_callback`: The routine sets a flag byte in a structure and, if a control word is zero, copies a value from one field to another within the same structure.
- `dequeue_serial_byte`: The routine dequeues a byte from a serial device queue.
- `initialize_serial_device_4`: The routine checks a flag in a device structure and, if clear, copies a value from one field to another, then calls another routine and invokes a function pointer stored in the structure.
- `clear_serial_status_flags`: The routine clears a flag in a byte at an offset from a pointer and copies a byte from another offset into a word at an offset from the same pointer.
- `update_serial_state_flags`: The routine updates a byte in a structure via a pointer, sets a flag bit in a nearby byte, conditionally modifies a global variable based on bits in a byte from the structure, and may set the global variable to a constant and return early or clear a word in the structure.
- `clear_flag_byte`: The routine stores a zero byte into a byte at a fixed offset from a pointer in A6.
- `set_serial_device_flags`: The routine checks a flag byte in a device structure and, if clear, sets the flag, clears a field, computes a timeout value from another field in the structure, stores the timeout and a command length, then calls another routine via a function pointer stored in the structure.
- `clear_serial_flag_and_dispatch`: The routine clears a bit in a byte field of a structure pointed to by a1 and, if a flag was set and a global variable matches a condition, sets another bit in the same byte, copies a value from one field to another within the structure when needed, and then calls another routine via a function pointer stored in the structure.
- `process_serial_input_2`: The routine reads a byte from the serial device, masks it with a stored mask, optionally clears a bit selected by a stored index, and compares the result with two stored bytes to set or clear a flag byte and a global variable.
- `set_serial_interrupt_flag`: The routine writes a constant to a global variable, sets a flag byte in a structure pointed to by an uninitialized pointer, and if a field in that structure is zero copies another field into it.

### Castlevania (1990)(Konami)[cr QTX].adf:cast

- `clear_table_and_dispatch`: The routine clears a block of memory to zero using a loop, stores a pointer and two integer values derived from a global variable into a local frame, and then conditionally calls another routine based on a flag in the global variable.
- `main_program`: The program's main routine: sets the start-up options (a command-line argument selects the RAM-disk mode), initialises the system, shows the title and the copy-protection screen, then runs the attract and game loops: per frame it updates timers, the player, the enemies and the screen until the player wins a stage, loses the last life or quits.
- `return_immediately`: The routine contains three instructions and ends with a return to its caller.
- `loop_until_threshold_met`: The routine repeatedly calls a function obtained from a table via a global pointer and compares its return value with a global variable.
- `clear_two_word_tables`: The routine clears a set of word entries in two adjacent tables by writing zero to each entry in a loop of thirty-two iterations.
- `clear_tile_table`: The routine clears a rectangular block of 16-bit entries in a global table.
- `clear_table_entries`: The routine clears a table of eighteen 16-bit entries by writing zero to each element.
- `clear_enemy_state_words`: The routine clears a block of fourteen 16-bit words in a global variable by storing zero to each consecutive location.
- `init_system`: Start-up: stores the copy-protection routine in a function pointer for the main routine to call, opens the libraries, installs the interrupt handlers, the audio and keyboard handlers, suppresses DOS requesters and loads the common sounds and graphics.
- `clear_state_flags`: The routine clears a block of ten consecutive word-sized fields in a global variable, then calls another routine, and finally sets two word-sized fields in the same global variable to one.
- `initialize_game_state_2`: The routine writes constants to several global variables and then loads a byte from a table indexed by a short offset, scales it, and stores the result in a global word.
- `copy_word_from_offset`: The routine calls another routine twice and then copies a 16-bit value from one offset of an address derived from a register to another offset of the same base.
- `initialize_global_state`: The routine writes constants to a global variable and two other global variables, then loads a byte from a table indexed by a short stored in a global variable into a global variable, and loads a second byte from a second table using an index derived from another global variable and the first loaded value into another global variable.
- `reset_game_state_flags`: The routine writes zero and one values to a set of word locations in a global structure and then calls another routine twice.
- `set_sprite_attributes`: The routine copies a byte from a table indexed by a short offset stored in a global variable into a word field of a global structure, then sets several word fields of that structure to constant values and a flag field to one, choosing between two values for one field based on the sign of the offset.
- `initialize_game_state_3`: The routine clears a number of word-sized fields in a global state structure, sets a few of them to one and one to a negative value, increments a counter field, and then calls another routine.
- `reset_game_state`: The routine writes zero to several word-sized fields in a global structure and sets two other word fields to one, then calls another routine.
- `set_title_screen_art`: The routine prepares a title screen by loading a shape table referenced by ART/TITSHAPE and a string referenced by No Game Saved, then runs a loop that reads input from a global variable and moves a cursor index between bounds using a byte table, calling drawing routines each iteration.
- `clear_state_flags_2`: The routine calls two other routines via pointers stored in memory relative to a base pointer, then clears five 16-bit words in memory relative to the same base pointer.
- `copy_state_to_save_buffer`: Copies the game state block that a saved game holds into the save buffer.
- `copy_state_fields_and_verify`: The routine copies a set of word and long fields from a block of memory to another block, then calls another routine with a global variable, a pointer into the block and a constant, and returns true if the call returns zero.
- `return_zero_constant`: The routine is empty and returns a constant zero.
- `load_castle_data`: The routine calls another routine with a global variable and a pointer derived from a4, then stores a sentinel value into a word in the a4 frame.
- `save_game_key`: Developer key in the game: when the player has energy left it shows SAVING GAME and saves the current game state through the save routine.
- `toggle_collision_flag`: The routine checks two short fields in a structure and, if both are non-zero, toggles a short flag between 1 and 0 and then calls another routine through a function pointer stored in the structure, passing a global variable and zero.
- `set_collision_off_flag`: The routine stores a constant into a global variable and then calls another routine with a global variable and a constant.
- `toggle_game_time_off_flag`: The routine checks two short flags in a global structure and, if both are non-zero, calls another routine with a global variable and a zero argument.
- `set_game_time_on_flag`: The routine calls another routine with a global variable and a constant, then stores a constant into a word at an address derived from a register.
- `toggle_life_count_off_flag`: The routine checks two short flags in a global structure and, if both are non-zero, calls another routine with a pointer to a global variable and an immediate zero.
- `set_life_count_on_flag`: The routine calls another routine with a global variable and a constant, then stores a constant into a word at an address derived from a register.
- `reset_game_state_2`: The routine writes a constant to a global variable and then calls two other routines, one with a global variable and a constant as arguments and one with no arguments.
- `set_screen_number`: The routine checks two global flags and, if set, calls another routine to write three values and calls another routine to fill a small stack buffer.
- `load_stage_enemy_art`: Loads the enemy shape files of the current stage: for each of up to nine entries of the stage table it builds the file name from the art directory and the enemy name, loads and decodes the file and stores a pointer to each animation frame it holds.
- `call_indirect_via_offset`: The routine loads an address computed from a global variable plus a constant offset and calls the routine found there.
- `load_castle_shapes_and_goodies`: The routine initializes a small stack buffer, loads two art blocks referenced by art/castshps and art/goodies, writes each block's base pointer into a global variable and copies the block's entries into two global tables.
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
- `call_indirect_via_table_2`: The routine loads a function pointer from a global table using an index derived from the argument and an offset, then calls the retrieved routine with a value taken from another global table indexed by the same argument.
- `call_if_flag_set`: Calls the routine behind a stage flag when that flag is set; otherwise does nothing.
- `compute_entity_score`: The routine computes a weighted sum from fields of a structure pointed to by a global base plus an index, then conditionally calls another routine with arguments derived from the structure and a global flag, and if a result is non-zero it calls a second routine to transform that result before returning the value.
- `update_table_entry`: If a flag in a global variable is set, the routine calls a function pointer twice, stores the second result in a global variable and increments a counter if the first result is non-zero.
- `process_input_events`: The routine clears pending input flags, builds a 16-entry table from a global table using offsets and masks, calls another routine with that table, then iterates the entries and conditionally calls another routine for each.
- `reset_counter_on_marker`: The routine loops a number of times given by a global variable.
- `decrement_counters_and_call_helpers`: The routine loops twice, each time decrementing a global counter until a helper returns non-zero or the counter reaches zero, calling other routines in between.
- `sync_game_state`: This routine synchronizes a pair of word-sized state variables, invoking other routines and a table lookup based on a short index, and updates several flags and counters while recursing until values converge.
- `process_level_tilemap`: Iterates over a tilemap buffer using a base pointer and row offsets, invoking a drawing routine for each cell and dispatching additional calls based on tile values and masks.
- `initialize_display_buffers`: The routine calls two routines via a table of function pointers, then iterates a nested loop calling a routine via a table of function pointers with an index argument, then calls another routine, and finally calls three more routines via a table of function pointers.
- `run_callback_grid`: The routine calls a setup routine through a pointer stored in memory, then invokes a callback routine for each entry in a 40 by 40 grid via a function pointer, and finally calls three cleanup or finalization routines through pointers stored in memory.
- `redraw_status_panel`: Redraws the status panel: the score, the player and enemy energy bars, the time, the hearts, the lives and the stage number, through the panel printing routines.
- `copy_value_and_loop_call`: The routine copies a short from one offset of a base pointer to another offset of the same base pointer.
- `decrement_counter_and_loop`: The routine reads a signed short from a global variable, decrements it by one, and then calls another routine repeatedly.
- `parse_script_bytes`: The routine walks a byte table using an index derived from a global variable and a loop count, reads three-byte groups and skips values 0, 1 and 2.
- `scan_tile_map_for_events`: The routine iterates over a range of rows defined by a global counter and walks a three-row tile map using a stride stored in a global variable.
- `set_flag_by_index`: The routine receives an index in the low byte of its parameter and writes the value 1 into one of five distinct word-sized locations in a global variable, selected by a switch on that index.
- `set_flag_in_memory`: The routine stores the constant 1 into a word located at a fixed offset from an address in A4, effectively setting a flag in a global variable.
- `set_flag_in_memory_2`: The routine stores the constant 1 into a word-sized location derived from an uninitialized pointer by subtracting a fixed offset.
- `set_flag_in_memory_3`: The routine stores the constant 1 into a word located at a fixed offset from a base pointer in memory.
- `set_flag_in_global`: The routine stores the constant 1 into a word located at a fixed offset from a global variable.
- `set_flag_in_global_2`: The routine stores the constant 1 into a word at a fixed offset from a global variable.
- `clear_flags_and_call`: The routine clears a specific word-sized global flag selected by a field of the input parameter and then calls another routine with a constant argument, returning its result.
- `check_enemy_type_flag`: The routine reads a short value from an offset of a global variable and tests whether it matches one of several specific enemy type identifiers.
- `increment_counter_and_update_offset`: The routine checks a global variable and a shifted global variable against thresholds and a zero flag.
- `decrement_timer_or_update_counter`: The routine checks a global variable and a shifted global variable and a flag.
- `check_and_update_state`: The routine checks several global variables and, depending on their values, either returns a constant or calls another routine and returns a different constant.
- `check_and_update_state_flags`: The routine reads several short global variables through an offset base, checks a combination of flag conditions, and conditionally calls another routine.
- `check_flags_and_increment_counter`: The routine reads three short flags from a global structure via a base pointer, increments a short counter in the same structure by a constant when certain flag combinations are present, calls another routine under those conditions, and returns 0, 6 or 8 depending on the flag values.
- `check_and_decrement_counter`: The routine checks several short values in a global structure and, if certain flags are clear, decrements a short counter by a fixed amount and calls another routine.
- `player_walk_and_jump`: Moves the player on the ground from the joystick: starts a jump or a stair climb when asked, otherwise steps left or right by a fixed amount each frame and keeps the walking animation going.
- `process_animation_frame`: The routine updates a small animation state machine using a global variable as a base pointer.
- `reset_game_state_on_condition`: The routine checks a global flag and the bits of an input parameter, and if the condition is met it clears a global word, sets another global word, calls another routine, sets a further global word, calls another routine with a constant argument, stores the returned word into two global words, and clears one of them.
- `decode_tile_attribute`: The routine computes an index from two parameters, looks up three bytes in a table using a global base pointer and offsets, and conditionally writes those bytes to a global short or combines them into a return value based on two global flags.
- `set_tile_collision_flags`: The routine writes constants to several global variables and, when the parameter is not -1, loads values from a table into globals using an index derived from the parameter and a lookup.
- `load_palette_from_table`: The routine computes an index from a global variable, clears a block of palette words in memory, then loads three words per entry from a table pointed to by a global pointer using a stride from a global variable.
- `copy_sprite_data`: The routine copies a series of three-byte chunks from a source pointer into a set of fixed offsets in a global structure, advancing the source pointer by a stride stored in the structure.
- `process_enemy_hitbox_table`: Iterates over a table of enemy entries and updates per-entry timers and hitbox flags.
- `reset_input_state_flags`: The routine clears a block of input-related word flags and then recomputes them from several global input state words, setting flags when particular bits are set or cleared.
- `update_game_state_flags`: The routine updates several short fields in a global structure based on other fields, increments and resets counters, and calls other routines to obtain values that are stored back into the structure.
- `set_tile_flags_from_map`: The routine reads a short entry from a table indexed by a parameter and uses it to compute two map offsets.
- `update_enemy_animation_frame`: The routine advances a per-entity animation counter and selects a new tile/frame index from a pattern table using the entity’s current animation state and a global pointer.
- `update_enemy_state`: The routine updates per-entity state stored in a table indexed by a parameter, using another routine for timing and a global variable for input.
- `update_enemy_animation_frame_2`: The routine updates per-entity animation and state fields using a global table and another routine.
- `update_enemy_state_table`: The routine updates a small table of three entries by comparing two short fields per entry, setting a flag and adjusting a value, then calls another routine conditionally based on a global flag and iterates over a set of sixteen structures to invoke another routine twice and possibly change a status field, and finally clears two global flags and two entries in a table if a flag is set.
- `find_tile_property_in_map`: The routine uses a global variable as a base pointer to locate a map table and a tile size, then walks the table row by row from a start column derived from the first parameter, stopping when a sentinel byte is found or the end of the row is reached.
- `find_free_slot_in_table`: The routine scans a small fixed-size table of 11 entries stored via a global base pointer.
- `collect_item`: Handles the player touching an item: finds the active item slot at that place, marks it taken and applies the item from the stage's item table (hearts, sub-weapons, whip upgrades, money bags and the rest).
- `accumulate_and_test_range`: The routine adds an incoming integer to a global variable and reads a value from a function pointer.
- `check_enemy_collision`: The routine iterates over a table of enemy records and, for each active entry, computes a bounding box from a sprite table and tests overlap against other active enemies.
- `initialize_stage_objects`: The routine sets up a global counter and a table index, then repeatedly calls another routine and checks a status field to update a counter and a timer value.
- `increment_and_clamp_counter`: The routine increments a short counter in a global variable and stores 2 into another global variable.
- `increment_counter_and_scan_table`: The routine increments a short counter by five and caps it at 99, then repeatedly calls another routine and scans a table of structures for entries with specific flag values.
- `apply_platform_collision_box`: The routine iterates over platform structures stored in a table and computes collision box values for a moving object.
- `initialize_enemy_spawn_table`: The routine calls another routine twice and then repeatedly scans a table of entries, checking flags in each entry and skipping entries that do not meet the conditions.
- `process_tile_collision_data`: The routine iterates over a set of tile structures and filters them using flag fields, then writes derived position and size values through a pointer.
- `initiate_object_initialization`: The routine initializes object fields by copying values from a table using indices derived from a global variable and then sets up bounding box coordinates via arithmetic on those fields.
- `execute_collision_checks`: The routine computes a vertical and horizontal interval from a global table entry and a global position, then iterates over ten objects, testing their bounding boxes against the interval and calling another routine for matches.
- `clear_entity_slot_and_schedule_updates`: The routine clears a set of bytes in two global buffers using offsets derived from a table entry, then recursively invokes itself with derived indices and marks the table entry as processed.
- `process_entity_state`: The routine processes an entry from a table of entity records indexed by the parameter.
- `scan_table_for_value`: The routine iterates over a table of sixteen short entries spaced seventy bytes apart in memory relative to a base pointer.
- `store_format_string_argument`: The routine passes a format string reference to another routine together with a global variable and a value read from an address derived from an uninitialized register.
- `call_other_routine_with_global`: The routine calls another routine, passing zero and the address of a global variable, then returns.
- `format_two_digit_number`: The routine calls another routine with the constant 8 and the address of a global variable.
- `format_two_digit_value`: The routine calls another routine with a constant and a pointer to a global variable, then returns.
- `call_other_routine_with_global_2`: The routine calls another routine, passing zero and the address of a global variable, then returns.
- `call_indirect_via_table_3`: The routine checks a global variable and, if non-zero, uses it as an index into a table of pointers to call another routine indirectly with a derived pointer argument and a constant.
- `check_state_and_call_handler`: The routine reads a short value from a global variable.
- `call_other_routine_twenty_times`: The routine sets a counter to five and repeatedly calls another routine four times per iteration while decrementing the counter to zero, then calls the other routine once more.
- `recursive_call`: The routine calls another routine and then returns.
- `call_indirect_from_table_2`: The routine loads a function pointer from a table via an offset from a global variable and calls it with a pointer to another global variable and a constant argument.
- `conditional_indirect_call`: The routine checks a field of its parameter against a constant and, if the condition holds, performs an indirect call through a global function pointer using a global variable as argument and a constant as second argument.
- `call_indirect_via_table_4`: The routine loads a function pointer from a table at an offset from a global variable and calls another routine with a global variable and a constant argument.
- `call_indirect_via_table_5`: The routine loads a function pointer from a table using an offset from a global variable and calls another routine with a global variable and a constant argument.
- `find_or_create_enemy_entry`: The routine scans a table of enemy definitions to find a matching entry or allocate a new one.
- `update_object_entry`: The routine calls another routine and, if the result is non-zero, computes an offset from a global variable and a scaled index, then writes several fields in the resulting structure: a shifted value from the parameter, a scaled value derived from the parameter, and clears a field while copying one field to another within the same structure.
- `clear_entity_flags`: The routine calls another routine with a constant and a parameter, then uses the returned short to compute an offset into a global variable.
- `select_enemy_sprite_and_reset_state`: The routine selects a sprite index based on a global flag and several tables and then calls another routine with that index.
- `set_entity_flags`: The routine calls another routine with a fixed first argument and the two parameters it received.
- `set_entity_flags_from_index`: The routine calls another routine with a fixed first argument and the first parameter, then checks the returned short.
- `decrement_counter_and_update_struct`: The routine checks a global counter and a table entry against a global limit.
- `process_entity_action`: The routine dispatches on a type field from the first argument and, for several types, calls another routine with derived parameters and clears or updates a set of per-entity slots in a global structure using the returned index.
- `update_enemy_movement_state`: The routine computes an index from a parameter and a global base, then reads and compares random values from a table of code pointers.
- `clear_enemy_state`: The routine calls another routine with a constant argument and, if the result is non-zero, computes an offset from a global variable and the result, then writes several 16-bit values into that computed location, copying one field from within the same structure and loading one field from another global location.
- `map_tile_lookup`: The routine indexes a global variable table using a global variable offset and the low word of the parameter to select a case, then compares the parameter's second byte against a set of constants and returns a small constant for matching pairs, otherwise returns zero.
- `map_input_code_to_value`: The routine reads a short from a register and returns a constant for two specific input codes, otherwise returns zero.
- `map_input_to_value`: The routine reads a short value from the stack and compares it against two constants.
- `map_input_to_value_2`: The routine reads a short from a register and maps it to a small value.
- `check_d4w_equals_5c`: The routine reads a short value from register D4 and compares it to a constant.
- `return_zero_2`: The routine is empty and returns zero.
- `process_enemy_spawners`: The routine checks several global state flags and, when conditions are met, calls other routines to create or update enemy objects with parameters derived from global variables and then copies flag words.
- `clear_enemy_slot`: The routine clears a per-entity slot in a table when the entity is not in a specific state.
- `check_table_for_value`: The routine iterates over a table of short values and calls another routine for each entry that is not equal to a specific constant.
- `iterate_table_and_call`: The routine loops over eleven entries in a table referenced via a global variable, computes each entry’s base address, checks three short fields for specific values and a non-equal value, and calls another routine for entries that satisfy the conditions.
- `process_object_type`: The routine adjusts an object type field using a table lookup and then dispatches based on the type value.
- `check_collision_flag`: The routine computes an index from a global base and a field of the parameter structure, then reads several short values from that indexed location.
- `lookup_tile_byte`: The routine computes an index from a base pointer, an offset and a parameter, then reads a single byte from a table using an offset derived from the parameter.
- `load_castlevania_title_assets`: The routine builds a table of pointers to five text resources and then repeatedly draws the title screen, waiting for input, and animates the title graphics by writing to screen memory via a blitter helper.
- `store_two_word_pairs`: The routine stores the low and high 16-bit halves of the first argument into two consecutive 16-bit fields of a global variable, and stores the low 16-bit half of the second argument into the next 16-bit field of the same global variable.
- `update_enemy_state_table_2`: The routine iterates over eight 10-byte entries in a table pointed to by param_1 and updates per-entry timers and counters.
- `load_castle_art_and_sounds`: The routine clears a few global variables, loads two groups of art resources referenced by ART/CASTvill and ART/ECAST and a sound effect referenced by sounds/lwvill2.eff, builds a table of pointers from those resources, and then iterates over two byte tables to trigger drawing and sound playback via indirect calls.
- `process_dialogue_table`: The routine walks two byte tables pointed to by a global variable and drives a display routine with indices taken from a second table, handling line breaks and a termination marker.
- `process_text_table`: The routine walks two parallel byte tables using an index, repeatedly invoking a set of function pointers stored in a structure and calling another routine with constant arguments.
- `poll_and_cleanup`: The routine repeatedly calls another routine with a constant argument until it returns non-zero or a counter expires, then calls three other routines in sequence and returns the remaining counter value.
- `call_three_routines_via_table`: The routine calls three other routines through a table indexed by an uninitialized value.
- `insert_high_score_entry`: The routine loads a score table from a file and, if the current score qualifies, inserts it into the sorted top ten list, shifting existing entries down and updating the on-screen score display.
- `process_animation_frames`: The routine clears a flag, optionally steps a counter with a signed offset while invoking three callback routines, then resets a pair of control words.
- `reset_animation_state`: The routine clears a global variable and, if a counter is non-zero, sets two word globals to a pair of values and steps a short offset by eight each iteration while calling three other routines from a table.
- `initialize_game_state_4`: The routine writes constant values to a few memory locations, then runs a short loop that decrements a word and calls three other routines via function pointers, and runs a second loop that decrements another word, calls a function with two constant arguments, calls another routine, then calls two more routines with the result and via function pointers, repeating a fixed number of times before clearing a word.
- `update_animation_state`: The routine repeatedly updates a global short by adding a scaled value, then calls three indirect routines and decrements a counter until it reaches zero.
- `call_indirect_table_entry_2`: The routine loads a function pointer from a global table using an index built from two global short values and a byte from another global table, then calls that function with an argument derived from a global short value.
- `reset_state_and_run_callbacks`: The routine clears a flag in a global variable and, depending on a mode value, initializes several global fields from another global value or zeroes them, then stores three control words.
- `reset_game_state_4`: The routine writes constants to fields in a structure pointed to by a register and to fields in a global variable, then loops a number of times determined by a field in the structure, updating a field in the global variable each iteration and calling three other routines via function pointers stored in the global variable.
- `accumulate_scaled_value_loop`: The routine repeatedly reads a short from a table pointed to by a pointer, scales it and adds it to a short stored in a global variable, then calls three other routines via function pointers stored in a global table.
- `reset_state_and_wait`: The routine clears several global variables, sets up a small set of state words, then calls a series of other routines to perform initialization.
- `stage_clear_sequence`: End-of-stage sequence: stops the play routines, plays the stage-clear tune, scrolls the screen on step by step while counting the remaining time into the score, and waits for a key or the end of the count.
- `wait_for_input_and_update_state`: The routine polls a delay routine in a loop, increments a counter and calls update helpers, then initializes several global variables and runs a timed sequence that alternates calls to drawing helpers with conditional updates, and finally waits for a condition or timeout while continuing updates before clearing state variables and calling cleanup routines.
- `run_animation_frame`: Repeatedly steps a set of animations driven by table entries and a delay counter, using indirect calls through function pointers stored in a table.
- `reset_game_state_5`: It loads two short values from a global table and dispatches them via a table of function pointers, then polls another function until it returns a specific value.
- `reset_game_state_6`: The routine calls two other routines via pointers stored in a table and clears three 16-bit fields in a global structure, setting them to zero.
- `set_spawn_flags_by_position`: The routine computes a position-derived index from two global variables and a room identifier, selects a case and sets a flag value based on ranges of the computed index and other global values.
- `set_spawn_flag_and_init_entity`: The routine computes a flag index from input values and, if the flag is clear, sets it and calls another routine to obtain an entity identifier.
- `spawn_entity_from_table`: The routine checks a set of input values and, when a condition holds, selects an entry in a table and marks it.
- `manual_password_check`: Copy protection (manual code lookup): shows a random grid reference (a letter from A to P and a row from 1 to 40), reads a typed number and compares it with that entry of the protection table XORed with a fixed key, allowing four tries.
- `password_retry`: Copy protection retry step: counts one more wrong answer and asks again.
- `set_timer_and_callback`: The routine stores a value returned by another routine into a global variable, then calls another routine with a fixed argument and a flag.
- `build_effect_script_table`: The routine builds a script table for sound effects using a helper routine and a set of string tables.
- `check_and_dispatch`: The routine tests a field of its parameter against a set of constant values.
- `handle_music_event`: The routine processes a music event parameter and updates a global state table.
- `wait_for_any_key`: Waits until the key-reading routine reports a key press.
- `empty_routine_3`: The routine contains no instructions and returns immediately.
- `fatal_error_exit`: Shows a fatal message (the copy-protection failure text for code 99, otherwise a numbered error) in the game's message box and leaves the program.
- `toggle_sound_state`: The routine toggles a global flag and then calls another routine with a short value taken from a global variable, followed by a call to a routine that receives a string literal and zero.
- `set_sound_off_flag`: The routine stores a constant into a global variable and, if a global flag is non-zero, calls another routine with a value taken from a global variable.
- `toggle_effect_on`: The routine toggles a global flag and, if a condition flag is set, calls another routine with a value taken from a global variable, then calls another routine with a constant and zero.
- `clear_effect_flag`: The routine sets a global variable to a constant and, if a global flag is non-zero, calls another routine with a value taken from a global variable.
- `pause_screen`: Pause screen: shows the pause banner (the text the Quartex crack put there), waits for a key and switches the developer cheat keys on when the cheat code was typed meanwhile.
- `check_stack_flag_and_dispatch`: The routine checks a short value passed on the stack.
- `conditional_callback_invoker`: The routine checks a short value on the stack and conditionally calls a callback through a pointer derived from a register.
- `conditional_callback_dispatch`: The routine checks a short value on the stack and, if it equals a specific constant, calls a callback through a pointer derived from a register with two fixed arguments; otherwise it calls another routine with a stack buffer and a constant, then calls the same callback with the buffer and a constant.
- `high_score_entry`: After a game over with a score high enough: shows the high-score table, lets the player type a name into the free place and saves the table to the score file.
- `initialize_memory_table_4`: The routine walks a linked list of entries to find the maximum value and writes that maximum into a global table, then fills a range of the table with an incrementing pattern.
- `initialize_memory_manager_2`: The routine stores an address in a global variable and writes several offsets derived from an allocation returned by another routine into a table at a fixed negative offset from a register.
- `set_global_word_and_dispatch`: The routine calls another routine, stores the constant 4 into a word at an offset from a base pointer, calls another routine twice, then calls a function pointer taken from an offset from the same base pointer.
- `copy_handler_table_entries`: The routine copies sixteen 16-bit entries from a table selected by the low byte of the argument to two destination tables and then calls another routine with a pointer to one of the destination tables.
- `swap_and_clear_buffer_entries`: The routine swaps two blocks of word entries in a global buffer area and then fills a small set of entries with a sentinel value.
- `clear_buffer_from_source`: The routine checks a global variable and, if non-zero, calls another routine with its address.
- `format_number`: Writes a number into a text buffer as decimal digits with a given width; a second entry takes a long value instead of a word.
- `format_number_with_sign`: The routine formats a signed 32-bit value into an ASCII decimal string using another routine for digit extraction, writing the sign if needed and terminating the string with a null byte.
- `signed_32_by_16_divide`: The routine divides a signed 32-bit dividend in d0 by a signed 16-bit divisor in d1 and returns a signed 48-bit result with the quotient in the high 32 bits and the remainder in the low 16 bits.
- `abs_small_value`: The routine receives a signed 16-bit value in a register, flips its sign if it is negative, and returns the absolute value as a 16-bit result.
- `absolute_value`: The routine receives an integer parameter and, if it is negative, replaces it with its negation.
- `random_below`: Returns a pseudo-random number from zero up to one less than the given limit, stepping the game's random generator.
- `seed_random`: Seeds the random generator from the CIA time-of-day and timer counters.
- `parse_format_string`: The routine walks a format string and for each conversion specifier it reads a value from a table of arguments and writes the formatted representation into a buffer, handling width, zero padding and signed decimal and hexadecimal conversions.
- `format_string_to_buffer`: The routine walks a format string character by character, handling escaped characters and percent specifiers.
- `format_string_with_args`: The routine parses a format string containing percent directives and writes formatted values into a stack buffer, using another routine to fetch arguments from a table and to output characters.
- `conditionally_call_another_routine`: The routine loads an unsigned byte from a global variable, adds a constant offset and checks the signed result against a negative threshold.
- `count_string_length`: The routine receives a pointer to a byte string and walks it byte by byte until it finds a zero byte, counting how many bytes precede the terminator.
- `draw_char_scrolled`: Draws one 8 by 8 character of the game font into the scrolling play screen (all five bitplanes, at the current scroll offset) in the current colours.
- `draw_char_panel`: Draws one 8 by 8 character of the game font into the fixed screen area, writing the same picture to all three screen buffers.
- `draw_char`: Common body of the two character routines: looks the character up in the font and writes its eight lines into each of the five bitplanes, setting or clearing the bits according to the foreground and background colour masks.
- `copy_word_pair_from_stack`: The routine copies two 16-bit words from a stack-relative source to a stack-relative destination.
- `copy_word_pair_to_global`: The routine copies a 32-bit value from the argument into two 16-bit words of a global variable, or copies two 16-bit words from that global variable to itself when the argument is negative.
- `set_up_int_server`: The routine obtains a handle from a global table, installs the same handler in a set of interrupt structures and repeatedly signals them until a call returns zero.
- `reset_game_state_7`: The routine clears several 16-bit fields in a global structure by writing zero and a full-mask value, then calls another routine twice before returning the combined value of the two input registers.
- `initialize_object_state`: The routine copies a 16-bit field from the argument into several 16-bit slots of a global structure, derives a mask by shifting the stored value right by three and clearing the low bit, writes that mask to two slots and clears two other slots, then calls another routine and returns the value it received in D0.
- `xor_byte_at_offset`: The routine XORs a 16-bit value stored at a fixed offset from an address in A4 with a constant.
- `store_word_into_global_offset`: The routine stores a 16-bit value taken from the first parameter into a global variable at a fixed negative offset from a register.
- `store_word_into_global`: The routine stores a 16-bit value from the first parameter into a global variable via a pointer derived from a register.
- `copy_palette_block`: The routine copies eight 16-bit words from a 32-bit source value stored in a global variable to two destination buffers referenced by global pointers, using an offset from a global short.
- `reset_dma_control`: The routine clears several words in a block pointed to by a register, writes a constant to a word, copies a long from a global location to another location, calls another routine twice, copies two longs from global locations to offsets in two different output pointers, calls another routine twice more, and then spins until a global variable’s bit is cleared.
- `reset_dma_channels`: The routine performs a busy-wait loop, copies two values from one area of memory to another, adds a short offset to them, clears and initializes several word fields, calls another routine, then waits until a global variable no longer has a specific bit set.
- `clamp_and_store_entry`: The routine looks up an entry in a table using a global index and a global counter, then clamps the two parameters to valid ranges and writes the clamped values into the entry.
- `populate_entity_slot`: The routine uses a global variable as a base to locate a table of slots and a counter.
- `draw_palette_block`: The routine processes a palette block descriptor pointed to by its argument.
- `iterate_table_and_call_2`: The routine walks a table of pointers taken from a global variable, calling another routine for each non-zero entry and storing a zero through a pointer.
- `find_free_slot_in_table_2`: The routine clears a global flag, selects one of two global tables based on a bit in the second argument, then scans up to eighty entries for a zero slot, writes the first argument into the first zero slot it finds, and marks the flag.
- `handle_game_state`: The routine manages a small state machine stored in a global variable, using a saved parameter and a callback pointer.
- `fade_and_wait`: Runs the palette fade step routine a number of times, then either waits about two seconds counted in frames or calls the routine supplied by the caller.
- `wait_and_process_state`: The routine waits in a tight loop, then calls another routine repeatedly, and then either processes a state value from a global variable by invoking another routine with a parameter and updating the state, or clears the state and calls another routine with fixed arguments followed by another routine with the result.
- `clear_flag_and_dispatch_2`: The routine clears a word in a global variable, calls another routine with two constants and then calls another routine with the result, then calls a function pointer stored in a global variable with a constant argument, and returns the value that was on the stack on entry.
- `decode_tile_row`: The routine repeatedly calls other routines until a condition on a returned word is met, then calls other routines again and finally invokes a routine with an input parameter.
- `parse_byte_stream_with_early_exit`: The routine walks a byte stream pointed to by a register, reading a length byte and advancing the pointer by that amount in a loop.
- `add_byte_to_table_entry`: The routine reads a short offset and a base pointer from two global variables, forms a pointer into a table, adds a byte from the argument to the byte at that entry and writes it back.
- `call_and_spin`: The routine calls another routine and then enters an infinite loop by comparing a byte from a global variable with itself, so execution never proceeds past the loop.
- `compute_sprite_scroll_offsets`: The routine reads a set of parameters and a type field, initializes several global variables, and conditionally updates scroll-related offsets and counters based on comparisons with global limits.
- `wait_for_vblank_and_copy_blitter_state`: The routine waits for a hardware flag to clear twice, then copies a series of values from a block of memory referenced via a base pointer into a global variable, repeatedly overwriting it.
- `adjust_scroll_and_redraw`: Moves the screen's scroll position back by the given amount, keeping it from going below zero, then redraws the screen edge.
- `process_collision_state`: The routine updates a set of global variables that track collision flags and a timer, writes values to a small block of memory used as an output structure, and calls another routine.
- `update_player_horizontal_position`: The routine updates a short horizontal position variable using a short velocity variable, clamps the position to a range, masks two flag words to the low two bits under certain conditions, and then derives a byte-sized position value by shifting the position right.
- `call_loop_with_return`: The routine invokes another routine, then repeatedly calls another routine with a value taken from a register and a previously returned value, followed by a call to another routine, looping while a word returned in a register is non-zero.
- `copy_sprite_tile_table`: The routine copies a series of bytes from a source table to a destination table using an index derived from registers and a global variable, and then writes constant byte values to several fixed offsets in the destination.
- `copy_palette_entries`: The routine copies a series of bytes from a source byte table pointed to by a pointer to a destination byte table pointed to by another pointer, using a fixed set of offsets.
- `store_param_in_global`: The routine stores the incoming parameter into a global variable by writing it through a pointer derived from a register.
- `copy_palette_bytes_to_vram`: The routine computes a source pointer from a byte field of the first argument and a base table, computes a destination offset from a short field of the first argument, a scaled value from the second argument, and two global values, calls another routine, then copies eight bytes from the source to eight fixed offsets in the destination area.
- `cia_timer_interrupt`: CIA-A timer A interrupt routine (about 48 times a second): runs the sound and music step of the game.
- `vblank_interrupt`: Vertical-blank interrupt server: counts frames, runs the registered per-frame tasks, swaps the displayed screen buffer when a new one is ready and updates the copper list.
- `swap_stack_pointers`: The routine checks a short value at a fixed offset from a base pointer and, if the condition is true, swaps the contents of three pairs of 32-bit words located at fixed offsets from the same base pointer.
- `swap_three_pairs_and_return`: The routine swaps the contents of three pairs of long words located at fixed offsets from a base pointer, then copies the value from the second swapped pair into a global variable.
- `fill_palette_entries`: The routine writes four 16-bit palette entries into two tables using a base pointer and an offset derived from global variables, computing each entry from a global integer and a short value with a constant added.
- `wait_for_vblank_and_clear_flags`: The routine waits until a short flag in a global structure becomes non-zero, optionally invoking a callback from a table if a global variable bit is clear and a local flag is not set, then invokes another callback from the same table, clears two word flags in the structure, and calls another routine twice.
- `decrement_until_negative`: The routine decrements a byte field in the argument and then repeatedly decrements it while it remains non-negative, using a loop that contains an empty spin on a byte from a global variable.
- `copy_word_table`: The routine copies 32 words from a source pointer to three destinations: a global table, another global table, and a third global table, using an index derived from a global variable.
- `store_indexed_table_entries`: The routine computes an index from the parameter and a global offset and writes the parameter into three tables referenced by global pointers, using different indexing schemes for each.
- `clear_and_copy_word_table`: The routine clears a global flag, then repeatedly calls another routine and spins until a global byte stops changing, repeating this twenty-one times.
- `clear_flag_and_poll_loop`: The routine checks a global flag and, if set, clears it and then repeatedly calls another routine while spinning on a byte comparison until a counter expires.
- `clear_word_table`: The routine clears a block of 32 words in a global table by writing zero to each entry, then calls another routine with the table base address.
- `blend_colour_component`: Inner step of the palette fader: scales one four-bit colour component by the fade level and puts it back into the colour value.
- `copy_word_block`: The routine copies a fixed number of 16-bit words from a source pointer to a destination buffer located at a fixed offset from a global variable.
- `init_blitter_copper_lists`: The routine fills two parallel tables with pointers and word values, writes several constants into them, then calls another routine twice and updates a global variable.
- `fill_fixed_point_table`: The routine writes a series of fixed-point values into two tables using two input values and two base pointers.
- `keyboard_interrupt`: CIA-A serial port interrupt routine: takes the key code the keyboard sent, does the handshake and passes it to the key handler.
- `process_keyboard_input`: The routine reads a byte from the CIA keyboard serial register, looks it up in a table indexed by a derived value from a global variable, and if the value is positive it calls another routine, compares the resulting byte to a sequence of values from a pointer-based table and possibly invokes another routine based on the match, then updates a global variable and returns the incoming registers concatenated.
- `keyboard_special_keys`: Part of the keyboard handler: turns the raw codes of the special keys it watches (cursor, return, backspace, escape and the function keys) into the characters the game puts into its key buffer.
- `incremental_table_store`: The routine checks a global counter stored in a global variable.
- `decrement_loop_table_call`: The routine loops while a short value stored at a fixed offset from a base pointer is non-zero, decrementing that short each iteration and calling a routine taken from a table indexed by the decremented value.
- `clear_global_word`: The routine stores a zero word into a global variable via an address derived from a register.
- `add_vblank_task`: Adds a routine and its argument to the list of up to sixteen tasks that the vertical-blank interrupt runs every frame.
- `open_library_and_call_handler`: The routine opens a library via OldOpenLibrary, stores the returned handle in a global variable, and then calls a function pointer from a table.
- `clear_global_variable`: The routine calls another routine twice and then stores zero into a global variable accessed via an offset from a register.
- `clear_global_flag`: The routine stores zero into a global variable accessed via an offset from a register.
- `poll_input_until_zero`: The routine calls another routine twice per iteration while the first argument is non-zero, storing the returned value and checking two specific high-byte markers to jump to a global variable.
- `read_joystick_and_update_state`: The routine reads a global variable that reflects the current input and button state, derives a directional mask from the joystick port and the fire buttons, and updates a short counter stored via an address in A4.
- `clear_two_word_variables`: The routine clears two 16-bit words located at fixed offsets from a base pointer.
- `pop_key`: Takes the next character from the keyboard ring buffer, or returns zero when it is empty.
- `store_value_in_circular_table`: The routine updates a circular index stored in a global variable and writes a 16-bit value from a register into a table of 16-bit entries indexed by that variable.
- `read_text_line`: Reads a line of typed text from the keyboard buffer into a string with a blinking cursor at a screen position: letters, digits and space are accepted up to the given length, backspace removes the last one and Return ends the line.
- `compare_and_call`: The routine compares two short values from registers and, if they differ, calls another routine.
- `compare_and_recurse`: The routine compares two short values and, if they differ, calls another routine.
- `compare_and_branch`: The routine loads two short values from registers and compares them.
- `check_cheat_code`: Developer cheat check: returns true when the first six keys in the keyboard buffer were 6, 6, 1, 0, 1 and 7, and empties the buffer.
- `clear_pointer_value`: Writes a zero word to the memory address held in register A0.
- `copy_string_and_call_func`: Copies a null-terminated string from a pointer in a structure to a fixed offset in another structure.
- `update_sprite_position`: The routine updates a sprite's position by adding velocity values to its coordinates, conditional on a flag state.
- `load_packed_file`: Loads a game file: opens it with dos.library, reads it whole into the file buffer and closes it, then Huffman-decodes it into its destination and advances the chip or fast memory allocation pointer; shows a disk error and exits when the file cannot be opened.
- `process_file_path_string`: This routine manipulates a character string by iterating through it and copying characters into a buffer, handling specific delimiter logic to parse or transform the path.
- `huffman_decode`: Decodes a packed file: reads the checksum and output length from its header, builds the code tree and then reads one bit at a time, walking the tree to each leaf and storing its byte until the output length is reached.
- `huffman_read_tree`: Reads the Huffman code tree of a packed file recursively from the bit stream: a one bit makes an inner node with two subtrees, a zero bit a leaf whose eight-bit symbol follows, least significant bit first.
- `process_input_state_2`: The routine sets a global flag, calls an initialization routine, and invokes another function to retrieve an input value.
- `execute_audio_sequence`: Sets a global flag to indicate activity, then executes a series of function pointers from two different tables, likely handling sound effects or music playback.
- `check_and_execute_callback`: Sets a global flag, executes a routine from a function table, and conditionally calls another routine based on the result.
- `call_function_pointer_2`: Loads a function pointer from a global variable and invokes it.
- `call_function_pointer`: Loads a function pointer from a global variable and executes the referenced routine.
- `wait_for_input_ready`: Sets a global flag to indicate a wait state, calls a routine to initialize or check input hardware, and loops checking an input status function while calling a processing routine.
- `process_input_state`: Sets a global flag, executes a sequence of function pointers from a table to query and process input, and clears the flag upon completion.
- `shape_frame`: Returns the address of frame n of a loaded shape file, walking the frame headers (each frame holds four bitplanes and a mask of the size given in its header) and filling in the frame's image and mask pointers.
- `sort_word_array_shell`: Sorts an array of 16-bit integers in ascending order using a shell sort algorithm.
- `suppress_dos_requesters`: Opens intuition.library and replaces its AutoRequest entry with a stub (under Forbid, fixing the library checksum) so that DOS error requesters never open; the game shows its own disk error texts instead.
- `handle_disk_error`: Checks the drive status and displays a 'No disk in drive' error message if the disk is missing or invalid.
- `display_disk_write_protected_error`: Checks a status code and displays a disk write-protected error message if the code does not match a specific value.
- `display_io_error_message`: Calls a routine to display an error message regarding read/write failures.
- `spin_cpu_cycles`: Executes nested decrement loops to waste processor time.
- `parse_signed_integer`: Skips leading whitespace and optional sign characters from a string, then converts subsequent ASCII digits into a signed 16-bit integer using a global lookup table to validate digit characters.
- `call_indirect_and_store_offset`: Calls a function pointer stored in a global table and adds a constant offset to the result.
- `store_high_word_global`: Extracts the upper 16 bits of a 32-bit parameter and stores them into a specific global variable offset from the A4 register.
- `initialize_library_context`: Clears a block of memory and stores pointers to a global structure and a hardware register.
- `return_immediately_2_2`: The routine performs no operations and returns immediately.
- `initialize_sprite_instance`: Allocates a sprite structure and initializes its flags, position, and type identifier.
- `parse_string_tokens`: Allocates memory and parses a source string into an array of token pointers, handling whitespace skipping and quoted string extraction.
- `truncate_and_pad_string`: Copies characters from a source buffer to a destination buffer up to a specified count or until a null terminator is encountered.
- `copy_string_with_limit_2`: Copies a null-terminated string from a source pointer to a destination pointer, truncating the copy after a fixed number of bytes if the source string is longer.
- `copy_string_with_limit`: Copies characters from a source string to a destination buffer up to a specified maximum count.
- `copy_null_terminated_string`: Copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `multiply_32_bit_integers`: Performs a 32-bit by 32-bit integer multiplication using 16-bit partial products.
- `open_console_streams`: C library start-up: opens the standard input and output streams of the program (the console or the window name given), storing the handles for the file routines.
- `store_and_call_indirect`: Stores a parameter into a global variable and calls another routine with that variable's address.
- `store_value_and_increment_pointer`: Stores a byte value from the input parameter into the memory address pointed to by a global pointer variable.
- `call_nested_handler`: Invokes another routine using a pointer passed as an argument and a stack location.
- `call_two_unnamed_routines`: The routine executes 48 instructions and calls two other unnamed routines.
- `parse_format_string_2`: Interprets a printf-like format string to process characters and arguments.
- `adjust_signed_value`: Determines a sign based on the comparison of two input registers and calls another routine.
- `adjust_sign_of_d1`: Calls another routine and conditionally negates the value in register D1 based on the sign of register D0.
- `call_and_return_d1`: Calls another routine and returns the 32-bit value in register D1.
- `call_recursive_routine_2`: Calls another routine and returns.
- `perform_32_bit_division`: Performs a 32-bit integer division of the value in D0 by the value in D1.
- `call_recursive_routine`: The routine calls another routine and returns.
- `call_three_subroutines`: The routine executes three calls to other unnamed routines.
- `return_immediately_2`: The routine is empty and returns immediately without performing any operations or modifying state.
- `call_two_subroutines_2`: The routine executes fifteen instructions and invokes two other unnamed routines.
- `fetch_byte_from_table`: Reads a byte from a table using an index stored in a global variable, increments the index, and returns the byte.
- `iterate_and_call_subroutine`: Calculates an offset from a global base pointer and iterates through a range of values.
- `clear_entity_flags_2`: Checks a byte at offset 3 of the input pointer for specific bit flags, calling other routines based on those bits.
- `clear_entity_state_flags`: Checks flag bits in a structure pointed to by a register and calls other routines based on those flags.
- `flush_file_buffer`: C library: writes out the buffered data of an output stream, or resets the stream when asked to discard it, and returns an error when the stream is not writable.
- `allocate_tile_memory`: Checks state flags and allocates a block of memory for a tile entity.
- `put_char_to_stream`: C library: puts one character into a stream's buffer, flushing it first when it is full or the stream is unbuffered.
- `process_tile_data`: Checks a global flag and conditionally updates memory pointers and byte values based on register states.
- `advance_pointer_and_store_byte`: Increments a pointer stored in a global variable and writes a byte from another global variable to the memory location pointed to.
- `find_empty_entry_in_table`: Iterates through a table of structures using a fixed stride, checking a specific byte offset for a null terminator.
- `find_empty_entry`: Iterates through a table of structures with a fixed stride, checking the fourth byte of each entry for a null terminator.
- `clear_three_words`: Sets three consecutive 32-bit words at a fixed global address to zero.
- `configure_object_state`: Checks a condition via an external call and sets object flags and pointers based on the result.
- `configure_entity_state`: Sets a word field to a fixed constant and updates a byte flag by ORing it with specific bits.
- `clear_linked_list`: Iterates through a linked list stored in a global variable, calling another routine for each node, and finally sets the list head to null.
- `traverse_linked_list`: Iterates through a linked list structure by following pointers until a null terminator is reached.
- `prepend_linked_list_node`: Allocates a new node via another routine and inserts it at the head of a linked list stored in a global variable.
- `update_pointer_chain`: Loads a pointer from a global variable and a second value from a register-based offset, then stores them into a linked structure.
- `recurse_param_lower`: The routine calls another routine passing the lower two bytes of its input parameter.
- `remove_linked_list_node`: Traverses a singly linked list stored in a global variable to find a node matching a given pointer.
- `free_memory_block`: C library: finds a block in the program's list of allocated memory, unlinks it and returns it to the system with FreeMem.
- `store_value_conditional`: Stores a value from a pointer into either a global variable or another pointer based on a condition, then calls another routine.
- `store_pointer_and_call`: Stores a value from a pointer into a global variable at a fixed offset.
- `check_entry_validity`: Checks if an index is within bounds and if a pointer in a table is non-zero.
- `check_table_entry_value`: Calculates an address by combining a base pointer from a global variable with an index scaled by six from a structure.
- `return_zero_constant_2`: The routine immediately returns a constant value of zero.
- `check_and_store_flag`: Calls another routine to check a condition.
- `return_zero_value`: Returns a constant zero value to the caller.
- `call_two_subroutines`: The routine executes nineteen instructions and invokes two other unnamed routines.
- `check_and_set_state`: Checks a flag in a global structure to determine if a specific condition is met.
- `check_and_store_result`: Calls another routine with arguments derived from global pointers and offsets.
- `do_nothing_stub_2`: The routine is empty and returns immediately without performing any operations.
- `check_flag_and_dispatch`: Calls another routine with specific arguments and checks if a bit is set in the return value.
- `call_subroutine_chain`: The routine invokes another routine and then returns zero.
- `call_indirect_function_7`: The routine invokes a function pointer stored at a fixed offset from a base register.
- `call_function_pointer_from_a4`: Checks a global variable offset from the A4 register; if non-zero, it treats the value as a function pointer and calls it.
- `process_entity_update_queue`: Iterates through a list of entities stored in a structure, calling an update routine for each.
- `execute_entity_update_loop`: Iterates through a count stored in a structure to call update subroutines for various fields.
- `check_and_invoke_callbacks`: Checks a flag in the global state structure to determine execution path.
- `call_routines_and_return_value`: Calls another routine twice, passing a value from a global variable on the second call.
- `restore_register_and_return`: The common exit of the routine before it: restores the saved register and returns to the caller.
- `validate_and_clear_entry`: Checks if an index is within bounds and if the corresponding table entry is valid and not marked with a specific flag.
- `check_and_clear_flag`: Checks if the high bit of a byte at an offset in a global pointer is clear; if so, calls another routine with the pointer value.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_entry`: Loads a function pointer from a global library structure and invokes it.
- `call_indirect_function_6`: Loads a function pointer from a global table using an offset and executes it.
- `call_indirect_function_table`: Loads a function pointer from a global table located at a negative offset from the A4 register.
- `indirect_call_via_a4_offset`: Loads a function pointer from a global table using an offset relative to register A4 and executes it.
- `call_function_from_table`: Loads a function pointer from a global table using an offset and executes it.
- `call_indirect_function_5`: Loads a function pointer from a global table and executes it.
- `call_library_function_2`: Loads a function pointer from a global library structure and executes it.
- `call_library_function`: Loads a function pointer from a global library structure and executes it.
- `call_indirect_function_4`: Retrieves a function pointer from a global variable and executes it via an indirect call.
- `call_function_via_table`: Loads a function pointer from a global table using an offset and executes it.
- `execute_indirect_function`: Loads a function pointer from a global table and executes it.
- `call_indirect_function_3`: Loads a function pointer from a global variable and executes it via an indirect call.
- `call_vtable_function`: Loads a function pointer from a global variable's virtual table and executes it.
- `call_global_function_pointer_2`: Loads a function pointer from a global variable and executes it.
- `call_indirect_function_2`: Loads a function pointer from a global table offset by a base register and executes it.
- `call_global_function_pointer`: The routine calculates an address by adding offsets to a global variable pointer.
- `call_indirect_function`: Loads a function pointer from a global variable and executes it via an indirect call.
- `call_library_entry_point`: Loads a function pointer from a global variable offset and executes it.
- `call_indirect_function_pointer_2`: Loads a function pointer from a global variable offset and executes it via an indirect jump.
- `call_indirect_function_pointer`: Loads a function pointer from a global variable and executes it via an indirect jump.
- `allocate_and_setup_context`: The routine initializes a block of memory pointed to by a register, storing a parameter and results from a called function.
- `parse_sample_header`: Validates audio format signatures and initializes sample playback state by copying header offsets and dimensions to local variables.
- `mute_audio_channels`: Checks a flag and iterates through audio volume registers to silence specific channels based on a bitmask.
- `negate_and_store_word`: Negates the lower 16 bits of the input parameter and stores the result into a specific global memory location.
- `check_flag_and_call`: Checks a specific global variable offset from register A4.
- `process_dma_interrupt_state`: The routine checks a state flag and updates counters based on threshold comparisons.
- `sound_channel_step`: Per-channel step of the sound-effect player: when the channel's effect is finished it starts the next queued sample or returns the channel to the music, otherwise it advances the effect's pitch and volume.
- `configure_dma_interrupts`: The routine configures DMA control and interrupt enable registers to manage hardware access permissions.
- `configure_audio_interrupts`: Iterates through audio channels to configure interrupt settings based on a global variable's bit flags.
- `update_interrupt_mask`: Calculates a bit mask based on a register value and stores it in a global variable and a memory location.
- `check_sprite_flags_loop`: Iterates through a table of structures, checking a specific byte offset for a null value or calling another routine.
- `process_joystick_input`: Reads joystick and fire button states from the CIAA parallel port A register.
- `calculate_scaled_offset`: Reads a value from an unspecified global variable, subtracts it from three, and multiplies the result by four.
- `parse_8svx_header`: Validates an 8SVX audio file format by searching for FORM, VHDR, and BODY chunks.
- `clear_tile_or_delay`: Checks a global flag; if zero, it executes a busy-wait loop decrementing a counter while setting a global variable to a constant.
- `process_audio_trigger`: Iterates through a table of audio events, checking flags and conditions to populate sound parameters into a hardware register structure.

### Castlevania (1990)(Konami)[cr QTX].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jt_clear_table_and_dispatch`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to clear_table_and_dispatch.
- `jt_main_program`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to main_program.
- `jt_clear_state_flags_2`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to clear_state_flags_2.
- `jt_reset_three_flags`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to reset_three_flags.
- `jt_read_screen_table_entry`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to read_screen_table_entry.
- `jt_read_player_controls`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to read_player_controls.
- `jt_call_and_return_result`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to call_and_return_result.
- `jt_add_and_call_table_function`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to add_and_call_table_function.
- `jt_self_4`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to self_4.
- `jt_process_input_events`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to process_input_events.
- `jt_reset_counter_on_marker`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to reset_counter_on_marker.
- `jt_initialize_display_buffers`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to initialize_display_buffers.
- `jt_run_callback_grid`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to run_callback_grid.
- `jt_load_castlevania_title_assets`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to load_castlevania_title_assets.
- `jt_load_castle_art_and_sounds`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to load_castle_art_and_sounds.
- `jt_insert_high_score_entry`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to insert_high_score_entry.
- `jt_process_animation_frames`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to process_animation_frames.
- `jt_reset_animation_state`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to reset_animation_state.
- `jt_reset_state_and_wait`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to reset_state_and_wait.
- `jt_stage_clear_sequence`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to stage_clear_sequence.
- `jt_set_spawn_flags_by_position`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to set_spawn_flags_by_position.
- `jt_manual_password_check`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to manual_password_check.
- `jt_set_timer_and_callback`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to set_timer_and_callback.
- `jt_build_effect_script_table`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to build_effect_script_table.
- `jt_check_and_dispatch`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to check_and_dispatch.
- `jt_handle_music_event`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to handle_music_event.
- `jt_toggle_sound_state`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to toggle_sound_state.
- `jt_high_score_entry`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to high_score_entry.
- `jt_initialize_memory_manager_2`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to initialize_memory_manager_2.
- `jt_set_global_word_and_dispatch`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to set_global_word_and_dispatch.
- `jt_copy_handler_table_entries`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to copy_handler_table_entries.
- `jt_format_number`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to format_number.
- `jt_abs_small_value`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to abs_small_value.
- `jt_call_other_routine_and_return`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to call_other_routine_and_return.
- `jt_format_string_with_args`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to format_string_with_args.
- `jt_copy_word_pair_to_global`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to copy_word_pair_to_global.
- `jt_reset_game_state_7`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to reset_game_state_7.
- `jt_initialize_object_state`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to initialize_object_state.
- `jt_store_word_into_global`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to store_word_into_global.
- `jt_reset_dma_channels`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to reset_dma_channels.
- `jt_populate_entity_slot`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to populate_entity_slot.
- `jt_draw_palette_block`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to draw_palette_block.
- `jt_handle_game_state`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to handle_game_state.
- `jt_compute_sprite_scroll_offsets`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to compute_sprite_scroll_offsets.
- `jt_self_7`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to self_7.
- `jt_store_param_in_global`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to store_param_in_global.
- `jt_copy_palette_bytes_to_vram`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to copy_palette_bytes_to_vram.
- `jt_wait_for_vblank_and_clear_flags`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to wait_for_vblank_and_clear_flags.
- `jt_decrement_until_negative`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to decrement_until_negative.
- `jt_clear_flag_and_poll_loop`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to clear_flag_and_poll_loop.
- `jt_clear_global_word`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to clear_global_word.
- `jt_add_vblank_task`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to add_vblank_task.
- `jt_open_library_and_call_handler`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to open_library_and_call_handler.
- `jt_clear_global_flag`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to clear_global_flag.
- `jt_poll_input_until_zero`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to poll_input_until_zero.
- `jt_read_joystick_and_update_state`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to read_joystick_and_update_state.
- `jt_clear_two_word_variables`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to clear_two_word_variables.
- `jt_pop_key`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to pop_key.
- `jt_read_text_line`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to read_text_line.
- `jt_update_sprite_position`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to update_sprite_position.
- `jt_load_packed_file`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to load_packed_file.
- `jt_process_file_path_string`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to process_file_path_string.
- `jt_process_input_state_2`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to process_input_state_2.
- `jt_execute_audio_sequence`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to execute_audio_sequence.
- `jt_call_function_pointer`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to call_function_pointer.
- `jt_calculate_chunk_end_pointer`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to calculate_chunk_end_pointer.
- `jt_sort_word_array_shell`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to sort_word_array_shell.
- `jt_suppress_dos_requesters`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to suppress_dos_requesters.
- `jt_parse_signed_integer`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to parse_signed_integer.
- `jt_initialize_library_context`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to initialize_library_context.
- `jt_copy_string_with_limit_2`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to copy_string_with_limit_2.
- `jt_copy_null_terminated_string`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to copy_null_terminated_string.
- `jt_multiply_32_bit_integers`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to multiply_32_bit_integers.
- `jt_call_nested_handler`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to call_nested_handler.
- `jt_negate_and_store_word`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to negate_and_store_word.
- `jt_check_flag_and_call`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to check_flag_and_call.
- `jt_clear_tile_or_delay`: Jump-table entry in the program's data hunk: the code calls it through the small-data base register and it jumps straight on to clear_tile_or_delay.
