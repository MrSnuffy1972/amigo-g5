# Game 1728 (vroom (1991)(lankhor)[a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01728). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904352 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 16 of 16 units identical (bytes and relocations), 39 raw-word lines |
| Functions named and described | pass, 84 of 84 named (0 author symbols), 84 of 84 described (7 by rule, 48 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1172 of 1172 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 13824 bytes, 2 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1750 of 1787 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 7331 |
| code | 5998 |
| data | 421001 |
| padding | 6 |
| unused | 470016 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Vroom (1991)(Lankhor)[a2].adf:bootblock | 1 | 15 | 1 |
| logged disk load from Vroom (1991)(Lankhor)[a2].adf boot-block load | 1 | 981 | 33 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 293 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 55 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 290 | 17 |
| Vroom (1991)(Lankhor)[a2].adf bytes 819200-820224 (code never loaded by a reference run) | 1 | 0 | 0 |
| Vroom (1991)(Lankhor)[a2].adf bytes 824832-826368 (code never loaded by a reference run) | 1 | 0 | 0 |
| Vroom (1991)(Lankhor)[a2].adf bytes 830976-832000 (code never loaded by a reference run) | 1 | 0 | 0 |
| Vroom (1991)(Lankhor)[a2].adf bytes 836608-838144 (code never loaded by a reference run) | 1 | 0 | 0 |
| Vroom (1991)(Lankhor)[a2].adf bytes 848384-849408 (code never loaded by a reference run) | 1 | 0 | 0 |
| Vroom (1991)(Lankhor)[a2].adf bytes 854016-855552 (code never loaded by a reference run) | 1 | 0 | 0 |
| Vroom (1991)(Lankhor)[a2].adf bytes 860160-861184 (code never loaded by a reference run) | 1 | 0 | 0 |
| Vroom (1991)(Lankhor)[a2].adf bytes 865792-867328 (code never loaded by a reference run) | 1 | 0 | 0 |
| Vroom (1991)(Lankhor)[a2].adf bytes 877568-878592 (code never loaded by a reference run) | 1 | 0 | 0 |
| Vroom (1991)(Lankhor)[a2].adf bytes 883200-884736 (code never loaded by a reference run) | 1 | 0 | 0 |
| Vroom (1991)(Lankhor)[a2].adf blocks 1602-1603 (bytes 820224-821248) (code never loaded by a reference run, load address unknown) | 1 | 153 | 2 |

## Cases handled

- vroom (1991)(lankhor)[a2]/Vroom (1991)(Lankhor)[a2].adf: custom-format (non-DOS) disk, 834 non-empty blocks: matched against track-loaded segments
- extended reference run ignored: it does not re-execute 4 of the 650 PCs the canonical run logged (2423 PCs logged; the harness no longer reproduces the game), so its PCs and RAM are not used
- vroom (1991)(lankhor)[a2]/Vroom (1991)(Lankhor)[a2].adf: the boot block reads 11776 bytes from disk offset 0 to an address (found statically); recovered as a unit
- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address (play-rs-m1500): 133 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 187 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- vroom (1991)(lankhor)[a2]/Vroom (1991)(Lankhor)[a2].adf: bytes 819200-820224 look like code no run loaded: recovered position-independently (load address unknown)
- vroom (1991)(lankhor)[a2]/Vroom (1991)(Lankhor)[a2].adf: bytes 824832-826368 look like code no run loaded: recovered position-independently (load address unknown)
- vroom (1991)(lankhor)[a2]/Vroom (1991)(Lankhor)[a2].adf: bytes 830976-832000 look like code no run loaded: recovered position-independently (load address unknown)
- vroom (1991)(lankhor)[a2]/Vroom (1991)(Lankhor)[a2].adf: bytes 836608-838144 look like code no run loaded: recovered position-independently (load address unknown)
- vroom (1991)(lankhor)[a2]/Vroom (1991)(Lankhor)[a2].adf: bytes 848384-849408 look like code no run loaded: recovered position-independently (load address unknown)
- vroom (1991)(lankhor)[a2]/Vroom (1991)(Lankhor)[a2].adf: bytes 854016-855552 look like code no run loaded: recovered position-independently (load address unknown)
- vroom (1991)(lankhor)[a2]/Vroom (1991)(Lankhor)[a2].adf: bytes 860160-861184 look like code no run loaded: recovered position-independently (load address unknown)
- vroom (1991)(lankhor)[a2]/Vroom (1991)(Lankhor)[a2].adf: bytes 865792-867328 look like code no run loaded: recovered position-independently (load address unknown)
- vroom (1991)(lankhor)[a2]/Vroom (1991)(Lankhor)[a2].adf: bytes 877568-878592 look like code no run loaded: recovered position-independently (load address unknown)
- vroom (1991)(lankhor)[a2]/Vroom (1991)(Lankhor)[a2].adf: bytes 883200-884736 look like code no run loaded: recovered position-independently (load address unknown)
- vroom (1991)(lankhor)[a2]/Vroom (1991)(Lankhor)[a2].adf: 11 logged loads cover 24576 bytes of the image
- vroom (1991)(lankhor)[a2]/Vroom (1991)(Lankhor)[a2].adf: blocks 913-913 (bytes 467456-467968) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.02)
- Vroom (1991)(Lankhor)[a2].adf blocks 1602-1603 (bytes 820224-821248) (code never loaded by a reference run, load address unknown): never-loaded code-like bytes recovered position-independently (load address unknown, 478 of 1024 bytes decode)
- vroom (1991)(lankhor)[a2]/Vroom (1991)(Lankhor)[a2].adf: blocks 1614-1615 (bytes 826368-827392) outside the filesystem hold data, not code (opcode density 0.037, entropy 5.03)
- vroom (1991)(lankhor)[a2]/Vroom (1991)(Lankhor)[a2].adf: blocks 1625-1626 (bytes 832000-833024) outside the filesystem hold data, not code (opcode density 0.057, entropy 6.06)
- vroom (1991)(lankhor)[a2]/Vroom (1991)(Lankhor)[a2].adf: blocks 1637-1638 (bytes 838144-839168) outside the filesystem hold data, not code (opcode density 0.035, entropy 4.51)
- vroom (1991)(lankhor)[a2]/Vroom (1991)(Lankhor)[a2].adf: blocks 1645-1649 (bytes 842240-844800) outside the filesystem hold data, not code (opcode density 0.052, entropy 5.12)
- vroom (1991)(lankhor)[a2]/Vroom (1991)(Lankhor)[a2].adf: blocks 1659-1661 (bytes 849408-850944) outside the filesystem hold data, not code (opcode density 0.044, entropy 5.02)
- vroom (1991)(lankhor)[a2]/Vroom (1991)(Lankhor)[a2].adf: blocks 1671-1672 (bytes 855552-856576) outside the filesystem hold data, not code (opcode density 0.037, entropy 5.25)
- vroom (1991)(lankhor)[a2]/Vroom (1991)(Lankhor)[a2].adf: blocks 1682-1683 (bytes 861184-862208) outside the filesystem hold data, not code (opcode density 0.059, entropy 6.23)
- vroom (1991)(lankhor)[a2]/Vroom (1991)(Lankhor)[a2].adf: blocks 1694-1695 (bytes 867328-868352) outside the filesystem hold data, not code (opcode density 0.035, entropy 4.72)
- vroom (1991)(lankhor)[a2]/Vroom (1991)(Lankhor)[a2].adf: blocks 1702-1702 (bytes 871424-871936) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.12)
- vroom (1991)(lankhor)[a2]/Vroom (1991)(Lankhor)[a2].adf: blocks 1716-1718 (bytes 878592-880128) outside the filesystem hold data, not code (opcode density 0.044, entropy 5.13)
- vroom (1991)(lankhor)[a2]/Vroom (1991)(Lankhor)[a2].adf: blocks 1728-1729 (bytes 884736-885760) outside the filesystem hold data, not code (opcode density 0.037, entropy 5.45)
- vroom (1991)(lankhor)[a2]/Vroom (1991)(Lankhor)[a2].adf: blocks 1737-1737 (bytes 889344-889856) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.12)
- 10 never-loaded region(s), 12800 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 391168 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `step_disk_head_to_track` (Vroom (1991)(Lankhor)[a2].adf boot-block load +an address to an address (11776 bytes)): Moves the disk head toward a requested track one step at a time, updating disk control port bits between steps, and then waits for a status bit to clear. Kind: disk head/track synchronization check. Pass path: the loop where bVar2 == bVar5 (the target track/head is reached) and the subsequent do-while loop completes without a hardware error/interrupt.
- `process_symbol_table_data` (Vroom (1991)(Lankhor)[a2].adf boot-block load +an address to an address (11776 bytes)): Checks bits 4 and 5 of a byte at an offset from a pointer in a0, and if bit 4 is set, calls another routine to modify a bit at a pointer, then enters a loop that compares two values and calls either another routine or a routine that clears a bit at a pointer based on the comparison results. Kind: disk hardware/track validation. Pass path: the instructions immediately following the call (sub_1846 -> sub_17DE), which proceed to configure hardware registers (d7=an address, a5=an address) for subsequent loading/execution..
- `write_disk_dma_length` (Vroom (1991)(Lankhor)[a2].adf boot-block load +an address to an address (11776 bytes)): The decompiler produced no pseudo-code, so this follows only the listed facts: the routine writes the disk length register, makes seven unnamed calls, and is called from two unnamed routines. Kind: disk hardware/track length check. Pass path: the branch at an address (bne.b an address) is skipped, proceeding to moveq #an address, d0.
- `initialize_system_state` (runtime segment an address (play-rs-m1500)): Disables interrupts, sets a byte at a memory location, calls two unnamed routines, clears a range of memory using a loop, fills two tables with pointers and zeros using loops, updates two global pointers, and finally copies a string of bytes to a memory location before returning. Kind: disk hardware/sync check. Pass path: execution continues to the next instruction after the u4__sub_0112() call.

## Functions


### Vroom (1991)(Lankhor)[a2].adf:bootblock

- `read_boot_track_then_halt`: Repeatedly issues a read request through the exec library's synchronous I/O call until the request's status byte clears, then issues a second device command.

### logged disk load from Vroom (1991)(Lankhor)[a2].adf boot-block load

- `disable_all_interrupts_and_dma`: Writes the value that clears every enable bit (bit 15 clear, the rest set) to the interrupt and DMA control registers three times, calls one unnamed routine, and then loops forever.
- `manage_interrupt_enable_bits`: The decompiler produced no pseudo-code, so this follows only the listed facts: the routine touches the interrupt enable and interrupt request registers, makes two unnamed calls, and is called from one unnamed routine.
- `patch_and_copy_timer_code`: Programs timer registers of the CIA chip, copies a block of routine code into RAM, and writes a series of instruction words, among them no-op and return-from-subroutine words, into that memory.
- `clear_interrupt_and_dma_enables`: Clears a status variable, calls one unnamed routine, clears the variable again, writes the value that clears all enable bits to the interrupt and DMA control registers, and returns.
- `call_then_set_status_negative`: Calls one unnamed routine, then sets the status variable to all ones and returns the register pair unchanged.
- `call_single_unnamed_routine`: No pseudo-code is available.
- `step_disk_head_to_track`: Moves the disk head toward a requested track one step at a time, updating disk control port bits between steps, and then waits for a status bit to clear.
- `select_side_and_step_head`: Stores a value into a buffer, sets or clears a control bit according to its low bit, steps the head toward a target one track at a time, and waits for a status bit to clear.
- `seek_head_by_byte_steps`: Calls a routine that returns a current value, steps toward a target byte one unit at a time until they match, and then waits for a status bit to clear.
- `wait_until_status_bit_clear`: Loops while the busy flag in the byte at the address register stays set, then returns.
- `set_control_bit_then_halt`: Clears the two low bits of the byte at the address register, sets bit 0, and then loops forever without returning.
- `set_bit_pair_then_halt`: Sets bit 1, clears bit 0 and sets bit 0 of the byte at the address register, then loops forever without returning.
- `stub_unavailable_pseudocode`: No pseudo-code is available.
- `decode_packed_stream_bits`: No pseudo-code is available.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `copy_block_to_low_memory`: Calls one unnamed routine, then copies a fixed block of 23 longwords from one address to a fixed low-memory address.
- `poll_timer_flag_return_byte`: Sets a countdown to nine and polls a status bit until it is set or the count runs out, leaves the status variable at a fixed value, and returns the low byte of the data register.
- `poll_timer_flag_with_timeout`: Sets a countdown to nine and polls a status bit until it becomes set or the count runs out, then leaves the status variable at a fixed value.
- `strobe_port_and_read_word`: Sets a control byte on the port, calls routines to read a value, and returns either a fixed flag or the high word of the register, depending on an input bit.
- `set_port_control_bits`: Sets the high bits of the port control byte to a fixed mask while keeping a middle group of bits, then returns.
- `process_symbol_table_data`: Checks bits 4 and 5 of a byte at an offset from a pointer in a0, and if bit 4 is set, calls another routine to modify a bit at a pointer, then enters a loop that compares two values and calls either another routine or a routine that clears a bit at a pointer based on the comparison results.
- `clear_bit_and_branch`: Clears bit 0 of the byte at the pointer in a0 and branches to another routine.
- `modify_bit_at_pointer`: Sets bit 1 of the byte at the address pointed to by a0, saves registers to the stack, clears bit 0 and then sets bit 0 of the same byte at the address in a0, and finally loads a hardware register address into a1.
- `write_disk_dma_length`: The decompiler produced no pseudo-code, so this follows only the listed facts: the routine writes the disk length register, makes seven unnamed calls, and is called from two unnamed routines.
- `read_signed_disk_blocks`: Reads disk blocks, comparing each header word with a fixed signature, and decodes each matching payload into memory with a marker-based run decoder until the byte count runs out.
- `find_block_and_copy_longs`: Searches a word table for a key, and on a match with a checked byte copies a 128-longword block into the record at that position.
- `call_routine_and_return`: Calls one unnamed routine and returns with whatever it left in the registers.
- `merge_odd_even_mfm_bits`: Combines a value with two longwords from memory by masking one of them and shifting another left by one bit, the way the odd and even bit streams of MFM data are merged.
- `build_and_decode_symbol_table`: Reads a packed header, builds a 256-entry table of values and code lengths, and decodes the stream by repeatedly swapping and indexing into that table until the output is complete.
- `update_global_pointers`: Calculates an offset from a value in d0, clears two global longwords, copies two words from a table into two other global locations, and then performs a loop that adds values from the table to a pointer before storing the result in a global longword and returning.
- `decode_compressed_data_stream`: Decodes a bitstream using a symbol table and a series of conditional expansion rules to write bytes into a destination buffer.
- `store_state_and_refresh`: Stores three incoming register values into a status area; if the first value differs from the stored one, it saves it and calls an unnamed routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `dispatch_interrupt`: Checks bit 4 and bit 3 of the interrupt status register, and if both are set, jumps to an address stored at an offset of 8 from a pointer loaded from a global variable, then returns from interrupt.
- `dispatch_interrupt_request_bits`: Reads the interrupt enable and request registers, testing the master enable bit and the request masks 64, 32, and 16 to dispatch on the first set bit through a handler call or a counter update.
- `check_interrupt_and_jump`: Saves registers d0 and a0, loads a word from a hardware register into d0, and if bit 7 and bit 14 are set, it writes 128 to a hardware register, updates a pointer from an offset of 24 in a0, and jumps to the address at that pointer.
- `test_bit_and_jump_alt`: Tests bit 8 of register d0; if set, it writes 256 to a hardware register, updates a pointer from an offset of 28 in a0, and jumps to the address at that pointer.
- `test_bit_and_jump`: Tests bit 3 of register d0; if set, it writes 1024 to a hardware register, updates a pointer from a global variable, and jumps to the address stored at that pointer.
- `jump_on_condition`: Tests bit 3 of a register, and if it is set, writes 1024 to a hardware register, updates a pointer from a global variable, and jumps to the address at that pointer; otherwise it returns.
- `restore_processor_state`: restores registers d0 and a0 from the stack and executes an exception return.
- `perform_multiple_or_operations`: performs a series of bitwise OR operations on register d0 with various immediate byte values, restores registers d0 and a0 from the stack, and executes an exception return.
- `read_cia_and_return`: reads a byte from a CIA register into d0, restores registers d0 and a0 from the stack, and executes an exception return.
- `handle_keyboard_input`: Reads a byte from the keyboard serial register, performs bitwise transformations, updates a global variable, and conditionally writes specific bytes to a table at offset a8a0, or increments/decrements two different global counters based on the input value, before returning via an exception return.
- `write_value_to_stack_pointer`: performs a bitwise OR operation on d0 with an immediate byte and writes the word value eight to an offset of 22 in the stack pointer area.
- `run_indexed_handler_once`: Tests a signed index and, when it is non-negative, stores a table entry, sets the index to a sentinel and calls through the entry; otherwise it calls one unnamed routine and clears the index.
- `dispatch_indexed_function_pointer`: Loads a 16-bit signed index from a global variable and, if non-negative, uses it to fetch a pointer from a table and jump to the address stored at an offset of 12 bytes from that pointer.
- `call_routine_and_clear_index`: When a signed index is non-negative, calls one unnamed routine and then sets the index to minus one.
- `write_stepped_longword_table`: writes a sequence of longword entries to a buffer, each holding a running value that increases by a fixed step and split into two 16-bit words.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `set_display_fetch_registers`: writes the bitplane control register and the display data fetch start and stop registers with fixed values.
- `perform_sequence_of_or_operations`: performs a sequence of six bitwise OR operations on register d0 using immediate byte values.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `read_joystick_and_mouse_ports`: Reads the joystick and mouse port registers JOY0DAT and JOY1DAT, the CIA port registers and POTGOR, and updates a small counter tracking the fire and right and middle buttons.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_global_values`: stores the longword value 7c174 into two different global variables and clears a third global variable before returning.
- `set_global_values`: stores the longword value 3458 into two different global variables and returns.
- `update_global_and_call`: Compares a register to a global variable, updates that global if they differ, and then calls another routine multiple times.
- `increment_global_in_loop`: Sets a global variable to the value in a1, then if a register is not 11, it calls another routine; otherwise it enters a loop that calls another routine 8 times while adding 15532 to a global variable and incrementing a counter.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `perform_bitwise_or`: Performs a bitwise OR operation on register d0 with zero.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 116 into d7, then merges 0 into d0, then merges 0 into d0.
- `set_port_value`: Calls another routine and then writes 65535 to a port at offset 58.
- `call_another_routine`: Calls another routine and then returns.
- `call_routine_and_set_global_flag`: Calls another routine and then stores 65535 into a global variable before returning.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_address_and_trap`: Saves registers d1-d7 and a0-a6, computes its own return address and stores it into a global variable, then executes trap zero to enter the operating system.
- `initialize_system_state`: Disables interrupts, sets a byte at a memory location, calls two unnamed routines, clears a range of memory using a loop, fills two tables with pointers and zeros using loops, updates two global pointers, and finally copies a string of bytes to a memory location before returning.
- `copy_structure_fields`: Calls two routines, then copies two 32-bit values from a structure in a6 into the memory locations pointed to by a0 and a0 plus two, incrementing the pointer twice.
- `set_values_to_max`: Stores a maximum signed word value into three consecutive word-sized locations relative to a pointer.
- `initialize_memory_values`: Calls two routines, then stores three maximum signed word values from a table into consecutive locations relative to a structure, copies two longwords into global variables, and clears a word at a fixed offset.
- `set_bit_and_store`: Sets bit 15 of register d0 and stores the result at the pointer in a0, then increments a0.
- `wait_for_byte`: Enters a loop that continues as long as the byte at offset 6 of the structure in a6 is non-zero.
- `modify_register_d0`: Performs several bitwise OR operations on register d0, including one that reads a byte from the pointer in a0 into d0.
- `clears_data_register_zero`: Performs a logical OR operation on data register zero with zero.
- `write_space_to_memory`: Writes a space character to a memory location relative to a global pointer.
- `check_hardware_and_set_value`: Checks certain bits in a hardware register and, if a condition is met, writes a value to a memory location relative to a pointer.
- `modify_byte_and_loop`: Performs bitwise operations on a byte in memory, updates a global variable based on another memory value, and executes a fixed-count loop.
- `find_string_null_terminator`: Searches through a string in memory until a null terminator or a specific condition is met, potentially calling another routine during the process.
- `copy_string_to_memory`: Compares a byte from a pointer in a1 against a table to find a match, then copies 7 blocks of 80 bytes from a source table to a destination buffer in a0 using an offset of 80.
- `update_string_and_format_numbers`: Updates a counter in a global variable based on a value in a table, performing bitwise operations and conditional increments, then iterates through a table to convert numeric values into ASCII digit strings stored in a global buffer and calls another routine for each character.
- `modify_registers`: Sets bit 7 and bit 6 of d5 and clears bit 7 of the byte pointed to by a0.
- `tiny_ori_2`: Small routine of 1 instruction that merges 230 into d5.

### Vroom (1991)(Lankhor)[a2].adf blocks 1602-1603 (bytes 820224-821248) (code never loaded by a reference run, load address unknown)

- `undefined_routine`: The routine contains data that does not decode as valid instructions and therefore performs no discernible operations.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
