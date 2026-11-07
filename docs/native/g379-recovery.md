# Game 379 (Parasol Stars): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00379). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903520 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 37 of 37 named (0 author symbols), 37 of 37 described (6 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 240 of 240 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 737 of 737 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 102045 |
| code | 2914 |
| data | 798033 |
| padding | 16 |
| unused | 512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Parasol Stars - Rainbow Islands 2.adf:bootblock | 1 | 44 | 3 |
| logged disk load from Parasol Stars - Rainbow Islands 2.adf boot-block load | 1 | 74 | 9 |
| logged disk load from Parasol Stars - Rainbow Islands 2.adf boot-block load | 1 | 205 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 414 | 15 |

## Cases handled

- Parasol Stars - Rainbow Islands 2/Parasol Stars - Rainbow Islands 2.adf: custom-format (non-DOS) disk, 1743 non-empty blocks: matched against track-loaded segments
- Parasol Stars - Rainbow Islands 2/Parasol Stars - Rainbow Islands 2.adf: the boot block reads 140800 bytes from disk offset 11264 to an address (found statically); recovered as a unit
- Parasol Stars - Rainbow Islands 2/Parasol Stars - Rainbow Islands 2.adf: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- runtime segment an address: 240 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Parasol Stars - Rainbow Islands 2/Parasol Stars - Rainbow Islands 2.adf: 2 logged loads cover 146432 bytes of the image
- Parasol Stars - Rainbow Islands 2/Parasol Stars - Rainbow Islands 2.adf: blocks 2-6 (bytes 1024-3584) outside the filesystem hold data, not code (opcode density 0.006, entropy 5.30)
- 2 packed track region(s), 750592 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `run_scroll_input_loop` (runtime segment an address): Loops adjusting a scroll offset in fixed steps while a step counter is within range, then checks input bits and branches to update state. Kind: disk hardware/sync check. Pass path: The loop continues to process input/scrolling if the hardware status (DAT_00dff00a) remains within valid bounds; if the check fails (e.g., DAT_00dff00a >= 4 or specific hardware flags), it enters an infinite loop or a trap state..

## Functions


### Parasol Stars - Rainbow Islands 2.adf:bootblock

- `load_segment_and_clear_palette`: Loads a disk segment with device I/O, clears the background colour register and the 32 palette registers, and enters supervisor state through the exec library.
- `stub_empty_return`: The decompiled body is empty, so the routine returns to its caller at once.
- `stub_empty_return_2`: The decompiled body is empty, so the routine returns to its caller at once.

### logged disk load from Parasol Stars - Rainbow Islands 2.adf boot-block load

- `make_three_helper_calls`: Makes three calls to unnamed routines and is referenced by one unnamed routine.
- `copy_block_backward_or_forward`: Copies a block of bytes either forward or, when the destination lies above the source, backward from the end.
- `copy_block_then_jump_indirect`: Copies a block of bytes forward from the source to the destination, then jumps through a pointer it could not resolve.
- `toggle_interrupt_and_cia_control`: Temporarily clears and then re-enables one interrupt bit, sets two CIA control registers, and stores a constant in a global variable.
- `timing_wait_then_reset_globals`: Makes several calls to unnamed routines, runs a timing wait on a global counter, then resets two globals.
- `fill_low_memory_with_pattern`: Fills 253 longword entries starting at a low memory address with a fixed pattern value, then stores a constant in a global variable.
- `store_two_global_constants`: Writes two fixed constants to a global variable in sequence, leaving the second.
- `set_dma_control_and_fields`: Stores a constant in a field of the structure, writes a fixed value to the DMA control register, and stores a filler value in a global variable.
- `reset_then_enable_interrupt_bit`: Clears all interrupt enable bits, stores a constant in a global variable, then enables a single interrupt bit.

### logged disk load from Parasol Stars - Rainbow Islands 2.adf boot-block load

- `copy_block_with_alignment_fixups`: Copies a block of data between buffers, adjusting the source and destination pointers to even addresses before copying runs of longwords.
- `copy_counted_runs_then_jump`: Repeatedly calls four unnamed routines and copies byte runs from source to destination in counted loops, ending with an indirect jump it could not resolve.
- `copy_literal_then_repeat_runs`: Copies a literal byte, then a run of bytes repeated from the source, in counted loops driven by calls to unnamed routines.
- `match_prefix_code_entry`: Walks a table of entries, comparing the masked bit buffer against each entry's pattern until one matches, then extracts a value from the matched entry's fields by shifting.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 255 into d0, then loads 16 into d1.
- `mask_and_conditional_call`: Masks a word with a mask register and, when one byte is less than a limit, calls an unnamed routine.
- `stub_empty_return_3`: The decompiled body is empty, so the routine returns to its caller at once.
- `read_four_bytes_into_register`: Reads four consecutive bytes from memory and shifts each into a data register, so the last byte read ends up in the low byte.
- `build_prefix_code_table`: Reads a count, fills a buffer of values from the stream, then builds a table of prefix codes by walking bit lengths and assigning code values bit by bit.
- `copy_words_to_stack_jump`: Copies ten words from a block of globals onto the stack, then jumps through a pointer it could not resolve.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_scroll_input_loop`: Loops adjusting a scroll offset in fixed steps while a step counter is within range, then checks input bits and branches to update state.
- `advance_table_cursor_on_timer`: When a timer counter is not one and reaches zero, it reloads the counter, advances a cursor through a fixed word table with wrap, and copies a word from the table into a global.
- `find_byte_in_table_record`: Searches a table of up to 73 bytes for the current byte, then uses the match position to compute a record address.
- `program_blitter_and_advance_column`: Swaps two buffer addresses, waits for the blitter to become idle, then programs the blitter control and pointer registers.
- `wait_blitter_then_set_masks`: Waits until the blitter is idle, then sets its mask registers and two modulo registers to fixed values.
- `draw_line_octant_pixels`: Works out the octant and step direction from the coordinate differences and steps along a line, writing pixel bits into a buffer.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `set_bit_from_register`: Sets one bit in a byte at its pointer, with the bit position taken from the low bits of a register.
- `set_bit_with_offset`: Sets a specific bit in a byte through a pointer using a value derived from a register and a bitwise mask.
- `tiny_ori_2`: Small routine of 1 instruction that merges 2 into d0.
- `tiny_ori_3`: Small routine of 1 instruction that merges 1 into d0.
- `tiny_ori_4`: Small routine of 1 instruction that merges 134 into d3.
- `stub_bad_instruction_data`: Contains a single bit-set instruction on the byte at the address register, using the low three bits of the data register as the bit index, followed by undecodable data.
- `tiny_ori_5`: Small routine of 1 instruction that merges 200 into d7.
- `set_bit_masked`: Performs a bitwise OR operation to set a bit in a byte at a position determined by a masked register value.
