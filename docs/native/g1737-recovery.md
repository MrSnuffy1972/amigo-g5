# Game 1737 (vroom - graphic and qwerty keyboard fix (1992)(yakoul)[req. vroom]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01737). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901129 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 37 of 37 named (0 author symbols), 37 of 37 described (2 by rule, 7 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 69 of 70 (98.57%) |
| Recovered without load address (counted in proof 1, flagged) | 522240 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 595 of 595 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 232 |
| code | 2244 |
| data | 107554 |
| padding | 1083 |
| unused | 790016 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Vroom - Graphic and Qwerty Keyboard Fix (1992)(Yakoul)[req. Vroom].adf:VROOM.TXT | 2 | 25 | 2 |
| Vroom - Graphic and Qwerty Keyboard Fix (1992)(Yakoul)[req. Vroom].adf:BOOT | 1 | 559 | 33 |
| Vroom - Graphic and Qwerty Keyboard Fix (1992)(Yakoul)[req. Vroom].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Vroom - Graphic and Qwerty Keyboard Fix (1992)(Yakoul)[req. Vroom].adf:VROOM#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- vroom - graphic and qwerty keyboard fix (1992)(yakoul)[req. vroom]/Vroom - Graphic and Qwerty Keyboard Fix (1992)(Yakoul)[req. Vroom].adf:VROOM: Action Replay II (ARPF) Data Cruncher (xfd)-packed, unpacked with ancient (94022 -> 522240 bytes)
- reference run ref ends at a 68000 address error (odd PC an address after an address): the harness halted there, nothing after it ran (68000 HALTED: address error at odd PC an address after an address, vector 3 an address is not a handler in RAM (the 68000 double-faults and halts); run ended at frame 0)
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- vroom - graphic and qwerty keyboard fix (1992)(yakoul)[req. vroom]/Vroom - Graphic and Qwerty Keyboard Fix (1992)(Yakoul)[req. Vroom].adf:VROOM#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 immediate
- 1 never-loaded region(s), 522240 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Vroom - Graphic and Qwerty Keyboard Fix (1992)(Yakoul)[req. Vroom].adf:VROOM.TXT

- `open_dos_library_then_close`: Opens a library through the system library table, and when the open fails the decompiled body ends in bad instruction data.
- `move_keyboard_data`: Copies longword values from multiple global variables and local memory locations onto the stack, then moves three values from the stack back to a register, and finally swaps the upper and lower words of that register.

### Vroom - Graphic and Qwerty Keyboard Fix (1992)(Yakoul)[req. Vroom].adf:BOOT

- `validate_and_launch_named_file`: Parses a file name from the command line, opens that file through the DOS library, and checks that its first four bytes match a fixed tag and that its size fits in memory.
- `disable_hardware_copy_loader`: Writes values that clear the DMA and interrupt enable bits, copies 2 KB of code into a low memory location and jumps there.
- `print_usage_message`: Writes the usage text 'aload filename' through the output library routine and returns zero.
- `copy_block_table_then_call`: Walks a table of destination pointers and byte counts, copying bytes from a source region into each destination until it reaches an end marker.
- `set_register_and_branch`: Sets register d0 to negative 256 and branches to another location.
- `copy_words_skipping_sentinel`: Walks a range of word offsets from a fixed start to a fixed end, skipping entries that equal a sentinel value and copying the other words through a helper.
- `relay_single_routine_call`: Calls one routine and returns.
- `do_nothing_return`: This routine contains no instructions and simply returns without modifying any data.
- `search_three_word_table`: Scans a table of three-word entries for the input word and returns on a match.
- `patch_shift_opcode_copy_blocks`: Chooses one of three instruction encodings, a no-operation or one of two shift instructions, based on a computed value, and stores it in a shared global.
- `swap_memory_blocks`: Swaps 486 longwords between two memory locations using a loop, then branches to another routine.
- `patch_shift_opcode_copy_blocks_2`: Chooses one of three instruction encodings, a no-operation or one of two shift instructions, based on a computed value, and stores it in a shared global.
- `unpack_backward_bitstream_block`: Decodes a packed block backwards from an end pointer using the bit-reader helpers.
- `decode_literal_bytes_backwards`: Decodes a count of literal bytes backwards by reading bits through the bit-reader helpers, then copies runs of earlier output in a loop until a final flag ends it.
- `decode_literal_bytes_entry`: Sets a count of eight, decodes that many bytes by reading eight bits each through the bit reader, and continues into the same literal and copy loop as its neighbour.
- `unpack_bitstream_control_step`: Decodes one control step of a bit-packed stream backwards, copying a run of earlier output or a literal byte through the bit-reader helpers.
- `unpack_packed_stream_entry`: Enters the packed-stream decoder after a run copy and continues the same control loop as its neighbour.
- `store_word_argument_to_global`: Stores its word argument into a shared global and returns.
- `execute_four_instructions`: This routine executes four instructions and then returns to its caller.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `unpack_and_patch_data`: Copies a block of data from one location to another using a loop, clears a global variable, calls unpack_backward_bitstream_block, writes several values to custom hardware registers, performs a 25-iteration loop to swap longwords between a table and another location, and finally jumps to an address from a table.
- `write_cia_register_bytes`: Copies table bytes into CIA register locations at a fixed stride, sets two interrupt control bits and a value, then sets a shared global to a fixed value and jumps through a pointer to a handler.
- `load_blitter_registers_from_globals`: Copies ten shared values into the blitter pointer, modulo, control and data registers.
- `poll_port_bit_set_flags`: Clears a flag bit in a shared port byte, then while an input bit is set, sets the flag and calls a helper.
- `wait_set_disk_control_flag`: Calls a helper repeatedly until it returns zero, then clears bit two in a shared byte and sets it again when a bit of a pointed-to byte is clear.
- `spin_forever_idle_loop`: The body is an infinite loop with no work, so once entered it never returns.
- `set_clear_control_bits_call`: Clears bits three and one of a control byte through a pointer, clears bit zero and then sets bits zero and three, and calls a helper routine.
- `set_control_bits_then_call`: Clears bit three of a control byte through a pointer, sets bit one, clears then sets bit zero, sets bit three, and calls a helper routine.
- `initialize_hardware_pointers`: Loads three addresses into registers a0, a1, and a5, then branches to another routine.
- `install_handler_and_fill_memory`: Copies a block of table words into fixed memory locations, stores a handler pointer, and sets the top bit on three custom chip registers.
- `enable_interrupt_then_fill_memory`: Sets the top bit on three custom chip registers to enable them, waits for a video frame flag, then fills a memory range with a constant word.
- `fill_range_and_restore_registers`: Fills a memory range with a constant word and then writes three saved values back into custom chip registers.

### Vroom - Graphic and Qwerty Keyboard Fix (1992)(Yakoul)[req. Vroom].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
