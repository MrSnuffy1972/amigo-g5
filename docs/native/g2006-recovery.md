# Game 2006 (paradroid 90 (1990)(hewson)[cr cls][t +3 twf]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02006). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (916064 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 22 of 22 named (0 author symbols), 22 of 22 described (1 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 118 of 118 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 24576 bytes, 5 functions |
| Independent decoder cross-check (Ghidra against capstone) | 713 of 1308 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 5998 |
| code | 4134 |
| data | 777929 |
| padding | 3 |
| unused | 128000 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Paradroid 90 (1990)(Hewson)[cr CLS][t +3 TWF].adf:bootblock | 1 | 24 | 1 |
| logged disk load from Paradroid 90 (1990)(Hewson)[cr CLS][t +3 TWF].adf boot-block load | 1 | 28 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 112 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 154 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Paradroid 90 (1990)(Hewson)[cr CLS][t +3 TWF].adf bytes 46592-57856 (code never loaded by a reference run) | 1 | 0 | 0 |
| Paradroid 90 (1990)(Hewson)[cr CLS][t +3 TWF].adf bytes 329728-331776 (code never loaded by a reference run) | 1 | 587 | 2 |
| Paradroid 90 (1990)(Hewson)[cr CLS][t +3 TWF].adf bytes 332288-343552 (code never loaded by a reference run) | 1 | 377 | 3 |

## Cases handled

- paradroid 90 (1990)(hewson)[cr cls][t +3 twf]/Paradroid 90 (1990)(Hewson)[cr CLS][t +3 TWF].adf: custom-format (non-DOS) disk, 1508 non-empty blocks: matched against track-loaded segments
- paradroid 90 (1990)(hewson)[cr cls][t +3 twf]/Paradroid 90 (1990)(Hewson)[cr CLS][t +3 TWF].adf: the boot block reads 3584 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 84 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 16 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 17 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- paradroid 90 (1990)(hewson)[cr cls][t +3 twf]/Paradroid 90 (1990)(Hewson)[cr CLS][t +3 TWF].adf: bytes 46592-57856 look like code no run loaded: recovered position-independently (load address unknown)
- paradroid 90 (1990)(hewson)[cr cls][t +3 twf]/Paradroid 90 (1990)(Hewson)[cr CLS][t +3 TWF].adf: bytes 329728-331776 look like code no run loaded: recovered position-independently (load address unknown)
- paradroid 90 (1990)(hewson)[cr cls][t +3 twf]/Paradroid 90 (1990)(Hewson)[cr CLS][t +3 TWF].adf: bytes 332288-343552 look like code no run loaded: recovered position-independently (load address unknown)
- paradroid 90 (1990)(hewson)[cr cls][t +3 twf]/Paradroid 90 (1990)(Hewson)[cr CLS][t +3 TWF].adf: 4 logged loads cover 28160 bytes of the image
- paradroid 90 (1990)(hewson)[cr cls][t +3 twf]/Paradroid 90 (1990)(Hewson)[cr CLS][t +3 TWF].adf: 3 track-loaded blocks found in recovered runtime segments and classified through them
- paradroid 90 (1990)(hewson)[cr cls][t +3 twf]/Paradroid 90 (1990)(Hewson)[cr CLS][t +3 TWF].adf: blocks 22-22 (bytes 11264-11776) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.68)
- paradroid 90 (1990)(hewson)[cr cls][t +3 twf]/Paradroid 90 (1990)(Hewson)[cr CLS][t +3 TWF].adf: blocks 24-89 (bytes 12288-46080) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.08)
- paradroid 90 (1990)(hewson)[cr cls][t +3 twf]/Paradroid 90 (1990)(Hewson)[cr CLS][t +3 TWF].adf: blocks 114-412 (bytes 58368-211456) outside the filesystem hold data, not code (opcode density 0.013, entropy 4.95)
- paradroid 90 (1990)(hewson)[cr cls][t +3 twf]/Paradroid 90 (1990)(Hewson)[cr CLS][t +3 TWF].adf: blocks 416-642 (bytes 212992-329216) outside the filesystem hold data, not code (opcode density 0.038, entropy 5.96)
- paradroid 90 (1990)(hewson)[cr cls][t +3 twf]/Paradroid 90 (1990)(Hewson)[cr CLS][t +3 TWF].adf: blocks 672-672 (bytes 344064-344576) outside the filesystem hold data, not code (opcode density 0.004, entropy 2.14)
- 1 never-loaded region(s), 11264 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 5 packed track region(s), 438272 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Paradroid 90 (1990)(Hewson)[cr CLS][t +3 TWF].adf:bootblock

- `boot_allocmem_then_io`: Boot block that calls the exec library to allocate memory and to run an I/O request through DoIO, filling in fields of the request structure first.

### logged disk load from Paradroid 90 (1990)(Hewson)[cr CLS][t +3 TWF].adf boot-block load

- `copy_global_block_to_pointer`: Copies 256 longwords from a global data area to a destination pointer in a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `read_mouse_and_joystick_ports`: Writes a fixed value to the potentiometer control register, reads the mouse and joystick port movement registers and the right and middle button register, and reads the CIA port with the fire buttons.
- `store_delta_fields_then_dispatch`: Computes two byte-sized differences from its two inputs and stores them in structure fields, along with a third field, then calls a handler through a method table slot.
- `load_table_entry_then_dispatch`: Looks up a longword in a global table using two bits of the input word as an index, stores it in a structure field, then stores a further value and calls a handler through a method table slot.
- `increment_counter_and_dispatch`: Increments a global counter and then calls a handler through a method table slot read from a pointer.
- `dispatch_pending_interrupt_bits`: Reads the interrupt request register, masks the pending bits with a fixed value, and calls up to three handlers through function pointers depending on the sign of successive bits.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_word_into_three_fields`: Stores one word into three fixed fields of a structure and returns.
- `set_sprite_pointers_from_table`: Copies a table of eight longword sprite pointer values into the sprite pointer registers, choosing between two sources by a bit in a structure field.
- `run_indirect_call_sequence`: A sequence of 86 instructions that makes indirect calls through pointers read from a structure, passing several copied values between them.
- `store_pointer_set_flag_bit`: Calls two methods through object table slots, stores a pointer into a field, sets the top bit of a byte at a pointer, and makes a further call.
- `call_one_routine_return_zero`: Calls one other routine and returns zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_bits_in_indexed_word`: ORs a fixed 16-bit pattern into a word at an address formed from a base register and an index register, then returns.
- `copy_three_words_into_structure`: Copies three consecutive words from a source pointer into three consecutive fields of a structure, then returns an all-ones value.
- `walk_list_calling_node_handler`: Walks a linked list starting from a pointer at offset 6 of a base structure, calling a handler stored at offset 16 of each node and following the next pointer at offset 6 until it is zero.
- `call_handler_at_offset`: Calls the routine stored at offset 20 from a base register and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### Paradroid 90 (1990)(Hewson)[cr CLS][t +3 TWF].adf bytes 329728-331776 (code never loaded by a reference run)

- `enter_raw_code_region`: Start of a 432-instruction raw code region that is never loaded by a reference run, with no symbols and only one listed call.
- `check_stride_limit_and_copy`: Compares a value against a limit field in a structure, takes a division by a stride field into account, and then copies longwords through pointers.

### Paradroid 90 (1990)(Hewson)[cr CLS][t +3 TWF].adf bytes 332288-343552 (code never loaded by a reference run)

- `call_method_and_decrement_counter`: Calls one method through an object slot, then decrements a counter field in a structure and traps when the decrement overflows.
- `call_handler_per_string_char`: Loops over the bytes of a string until it reaches a space or a countdown runs out, calling a handler for each byte through a table slot.
- `copy_name_chars_until_delimiter`: Copies characters from a source buffer into an output buffer, stopping at a zero byte, a backslash, a dot or an asterisk, or when a counter runs out.
