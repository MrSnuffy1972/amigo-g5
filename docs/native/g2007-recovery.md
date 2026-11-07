# Game 2007 (Paradroid 90): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02007). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (916064 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 22 of 22 named (0 author symbols), 22 of 22 described (1 by rule, 2 by the local model) |
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
| Paradroid 90.adf:bootblock | 1 | 24 | 1 |
| logged disk load from Paradroid 90.adf boot-block load | 1 | 28 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 112 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 154 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Paradroid 90.adf bytes 46592-57856 (code never loaded by a reference run) | 1 | 0 | 0 |
| Paradroid 90.adf bytes 329728-331776 (code never loaded by a reference run) | 1 | 587 | 2 |
| Paradroid 90.adf bytes 332288-343552 (code never loaded by a reference run) | 1 | 377 | 3 |

## Cases handled

- Paradroid 90/Paradroid 90.adf: custom-format (non-DOS) disk, 1508 non-empty blocks: matched against track-loaded segments
- Paradroid 90/Paradroid 90.adf: the boot block reads 3584 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 84 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 16 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 17 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Paradroid 90/Paradroid 90.adf: bytes 46592-57856 look like code no run loaded: recovered position-independently (load address unknown)
- Paradroid 90/Paradroid 90.adf: bytes 329728-331776 look like code no run loaded: recovered position-independently (load address unknown)
- Paradroid 90/Paradroid 90.adf: bytes 332288-343552 look like code no run loaded: recovered position-independently (load address unknown)
- Paradroid 90/Paradroid 90.adf: 4 logged loads cover 28160 bytes of the image
- Paradroid 90/Paradroid 90.adf: 3 track-loaded blocks found in recovered runtime segments and classified through them
- Paradroid 90/Paradroid 90.adf: blocks 22-22 (bytes 11264-11776) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.68)
- Paradroid 90/Paradroid 90.adf: blocks 24-89 (bytes 12288-46080) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.08)
- Paradroid 90/Paradroid 90.adf: blocks 114-412 (bytes 58368-211456) outside the filesystem hold data, not code (opcode density 0.013, entropy 4.95)
- Paradroid 90/Paradroid 90.adf: blocks 416-642 (bytes 212992-329216) outside the filesystem hold data, not code (opcode density 0.038, entropy 5.96)
- Paradroid 90/Paradroid 90.adf: blocks 672-672 (bytes 344064-344576) outside the filesystem hold data, not code (opcode density 0.004, entropy 2.14)
- 1 never-loaded region(s), 11264 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 5 packed track region(s), 438272 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Paradroid 90.adf:bootblock

- `boot_allocmem_then_io`: Boot block that calls the exec library to allocate memory and to run an I/O request through DoIO, filling in fields of the request structure first.

### logged disk load from Paradroid 90.adf boot-block load

- `copy_global_block_to_pointer`: Copies 256 longwords from a global data area to a destination pointer in a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `read_mouse_and_joystick_ports`: Writes a fixed value to the potentiometer control register, reads the mouse and joystick port movement registers and the right and middle button register, and reads the CIA port with the fire buttons.
- `store_delta_fields_then_dispatch`: Computes two byte-sized differences from its two inputs and stores them in structure fields, along with a third field, then calls a handler through a method table slot.
- `load_table_entry_then_dispatch`: Looks up a longword in a global table using two bits of the input word as an index, stores it in a structure field, then stores a further value and calls a handler through a method table slot.
- `increment_counter_and_dispatch`: Increments a global counter and then calls a handler through a method table slot read from a pointer.
- `dispatch_pending_interrupt_bits`: Reads the interrupt request register, masks the pending bits with a fixed value, and calls up to three handlers through function pointers depending on the sign of successive bits.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_value_into_three_words`: It writes the same 16-bit value from a register into three consecutive 16-bit words of a structure, using fixed offsets from a base pointer.
- `copy_sprite_pointer_block`: Picks a 32-byte source block from one of two pointers, depending on a flag bit in the structure, and copies it into the eight sprite pointer registers of the custom chip.
- `initialise_object_slot_fields`: Calls several methods through object tables, stores the returned values and a masked flag word into fields of the object record, and decrements a counter with a signed overflow check.
- `store_object_pointer_and_flag`: Calls methods through object tables, stores the returned object pointer into a location passed by the caller, and sets the top bit of a byte at that location.
- `forward_call_and_return_zero`: Calls one unnamed routine and returns zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_bits_in_offset_word`: It reads a 16-bit word from a location addressed by a base pointer plus a register offset, bitwise-ORs it with a constant, and writes the result back to the same location.
- `store_word_triple`: Copies three consecutive 16-bit values from a source pointer into three consecutive 16-bit fields of a structure at fixed offsets from its base, then returns minus one.
- `call_callbacks_along_list`: Walks a singly linked list whose head pointer is stored in the structure, calling a routine stored in each node, and stops when a node's link field is zero.
- `call_indirect_via_table`: Calls the routine whose address is stored at a fixed offset in a structure pointed to by a base register, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### Paradroid 90.adf bytes 329728-331776 (code never loaded by a reference run)

- `stub_bad_instruction_data`: The decompiler reduced this 432-instruction raw region to a single call to an unresolved address, so the recovered code shows no usable behaviour.
- `copy_indexed_block_row`: Checks an index against a stored limit, splits it into a row and a column using a stride, and copies a row of eight-long blocks between the source pointer and the destination, in one direction or the other according to a flag argument.

### Paradroid 90.adf bytes 332288-343552 (code never loaded by a reference run)

- `decrement_counter_via_method`: Calls a method through an object reference stored in the structure, then decrements a 16-bit counter in the object it returns, calling the trap routine on signed overflow.
- `call_per_char_until_space`: Walks a character string one byte at a time, calling a routine from a table for each byte, and stops at the first space character or when a count held in a register runs out.
- `copy_name_into_fixed_field`: Copies a name into a fixed-width field, stopping at a dot, backslash, asterisk or end of string, then skips the remainder of the name.
