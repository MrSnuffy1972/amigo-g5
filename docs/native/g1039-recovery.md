# Game 1039 (leander (1991)(psygnosis)(disk 1 of 3)(disk 1 of 3)[cr skr][t icepic]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01039). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904096 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 41 of 41 named (0 author symbols), 41 of 41 described (2 by rule, 9 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 545 of 545 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 783 of 783 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 10808 |
| code | 2586 |
| data | 889678 |
| unused | 1024 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Leander (1991)(Psygnosis)(Disk 1 of 3)(Disk 1 of 3)[cr SKR][t Icepic].adf:bootblock | 1 | 48 | 2 |
| logged disk load from Leander (1991)(Psygnosis)(Disk 1 of 3)(Disk 1 of 3)[cr SKR][t Icepic].adf boot-block load | 1 | 93 | 3 |
| logged disk load from Leander (1991)(Psygnosis)(Disk 1 of 3)(Disk 1 of 3)[cr SKR][t Icepic].adf boot-block load | 1 | 207 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 435 | 16 |

## Cases handled

- leander (1991)(psygnosis)(disk 1 of 3)(disk 1 of 3)[cr skr][t icepic]/Leander (1991)(Psygnosis)(Disk 1 of 3)(Disk 1 of 3)[cr SKR][t Icepic].adf: custom-format (non-DOS) disk, 1755 non-empty blocks: matched against track-loaded segments
- leander (1991)(psygnosis)(disk 1 of 3)(disk 1 of 3)[cr skr][t icepic]/Leander (1991)(Psygnosis)(Disk 1 of 3)(Disk 1 of 3)[cr SKR][t Icepic].adf: the boot block reads 2048 bytes from disk offset 899072 to an address (found statically); recovered as a unit
- leander (1991)(psygnosis)(disk 1 of 3)(disk 1 of 3)[cr skr][t icepic]/Leander (1991)(Psygnosis)(Disk 1 of 3)(Disk 1 of 3)[cr SKR][t Icepic].adf: the boot block reads 11264 bytes from disk offset 0 to an address (found statically); recovered as a unit
- runtime segment an address: 351 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 4 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 3 immediate, 1 jump target
- leander (1991)(psygnosis)(disk 1 of 3)(disk 1 of 3)[cr skr][t icepic]/Leander (1991)(Psygnosis)(Disk 1 of 3)(Disk 1 of 3)[cr SKR][t Icepic].adf: 2 logged loads cover 13312 bytes of the image
- 1 packed track region(s), 886784 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Leander (1991)(Psygnosis)(Disk 1 of 3)(Disk 1 of 3)[cr SKR][t Icepic].adf:bootblock

- `boot_entry_allocate_memory`: Entry point reached from the disk boot strap, which requests memory from the Amiga exec library and calls one further unnamed routine.
- `configure_display_parameters`: Sets several values in a memory block pointed to by a register and updates a hardware register.

### logged disk load from Leander (1991)(Psygnosis)(Disk 1 of 3)(Disk 1 of 3)[cr SKR][t Icepic].adf boot-block load

- `write_decrementing_global_until_wrap`: Stores a counter that starts at minus one and decrements until it wraps around, then returns minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from Leander (1991)(Psygnosis)(Disk 1 of 3)(Disk 1 of 3)[cr SKR][t Icepic].adf boot-block load

- `modify_memory_and_disable_blitter`: Modifies two bytes in memory via a pointer in a4 by performing a bitwise OR with 124, calls a routine at an offset from a6, calls relocate_block_and_jump, and sets the bitplane enable bit in the BPLCON0 register to 0 before returning.
- `relocate_block_and_jump`: Writes interrupt and DMA control values, copies 544 bytes of code to a fixed address, and jumps into the copy through a pointer.
- `decrunch_backward_bitstream_block`: Moves a packed source block to the end of its buffer, then reads a bitstream from the end and writes literal bytes and repeated runs backwards into the output.
- `call_unresolved_indirect_target`: Makes an indirect call through a pointer whose target the decompiler could not recover, then returns.
- `store_byte_then_indirect_call`: Stores one byte just below an address register, then makes an indirect call; a zero test on another register selects between two call paths, both through pointers the decompiler could not recover.
- `dispatch_on_flag_indirect_call`: Makes one of two indirect calls, chosen by whether a flag byte is nonzero.
- `forward_to_unresolved_jump_target`: Calls through an indirect pointer whose target the decompiler could not recover, then returns.
- `dispatch_byte_threshold_indirect_call`: Chooses between two indirect calls by comparing a byte with six: values below six make one call, and other values make a second call, with the targets unrecovered in the decompiled code.
- `dispatch_on_value_above_one`: Makes one of two indirect calls depending on whether a byte is greater than one.
- `execute_indirect_jump_target`: Performs an indirect call by fetching a destination address from a jump table and jumping to it.
- `branch_on_flag_status`: Checks a byte and performs an indirect call based on whether that byte is non-zero.
- `execute_indirect_jump`: Performs an indirect jump to an unresolved target address.
- `dispatch_byte_below_three_call`: Makes one of two indirect calls: a byte below three selects the first target, and any other value selects the second.
- `dispatch_byte_above_two_call`: Makes one of two indirect calls: a byte above two selects one target, and any other value selects a second.
- `branch_on_threshold_value`: Checks if a byte is greater than two and performs an indirect call based on the result.
- `execute_indirect_jump_table`: Performs an indirect call by fetching a destination address from a jump table.
- `dispatch_indirect_call`: Evaluates a byte to determine which of two indirect calls to execute from a jump table.
- `execute_indirect_jump_2`: Performs an indirect jump to an address retrieved from a table.
- `copy_block_downward_then_call`: Copies a block of bytes one at a time, moving downward through memory from its end, then makes an indirect call whose target could not be recovered.
- `decrunch_backward_bitstream_block_2`: Unpacks a packed block like its neighbour: it refills a bit buffer from the end of the source, writes literal bytes and repeated runs backwards into the output, then makes an indirect call whose target could not be recovered.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `trap_zero_and_return`: Saves one global value, stores a routine address in another global, raises software trap zero, and returns zero.
- `open_library_and_wait_for_status`: Opens a system library through the exec library, reads a value from it and calls a second library routine.
- `copy_vector_block_to_low_memory`: Copies 147 bytes from the start of this code block to a fixed low-memory address, one byte at a time, and returns once the copy completes.
- `stub_undecompiled_routine`: The decompiled body timed out, so only the facts are available: 19 instructions, one caller and no listed calls or registers.
- `trap_zero_after_flag_check`: If a flag word is not one, calls a helper first.
- `advance_scrolltext_character`: Advances a per-frame text printer: every eighth frame it toggles a state flag, then for each character code it writes glyph pointers into output bitplane slots.
- `tiny_ori`: Small routine of 1 instruction that merges 157 into d0.
- `tiny_ori_2`: Small routine of 1 instruction that merges 1 into d0.
- `advance_scrolltext_flagged`: Runs the same per-frame text printer as its neighbour, but the toggle flag is read from a fixed address and a counter sets or clears one status bit at each of five steps as text codes are read.
- `advance_scrolltext_toggled`: Runs the same per-frame text printer as its neighbour, with an eight-frame toggle, and sets or clears status bits from a counter while it processes the text codes.
- `print_connect_status_text`: A long text-printing routine that handles the same character-code cases as the scroll printer, with a modem CONNECT-style message among its referenced text.
- `write_glyph_into_eight_rows`: Copies one byte from each of eight glyph-table rows, spaced 80 bytes apart, into eight output rows spaced 40 bytes apart, then calls a helper.
- `write_glyph_column_from_register`: Copies the same eight-row glyph column as its neighbour, but takes the table index from the D0 register, then calls a helper.
- `start_audio_channel0_sample`: Points audio channel zero at a sample buffer, sets its length to 64, period to 640 and volume to 40, then enables the channel's DMA. All values are fixed constants in the code.
- `busy_wait_counter_then_set_one`: Counts a 16-bit global value down until it wraps around, then sets a global flag to one.
- `decode_keyboard_serial_byte`: Rotates a keyboard byte from the serial register by one bit, inverts it, and keeps only two specific key codes, zeroing any other value.
