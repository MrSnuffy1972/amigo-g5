# Game 1648 (seek & destroy (1993)(mindscape)[cr cls](disk 1 of 4)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01648). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (925456 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 180 raw-word lines |
| Functions named and described | pass, 71 of 71 named (0 author symbols), 71 of 71 described (8 by rule, 54 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1011 of 1011 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2500 of 2500 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 16098 |
| code | 9626 |
| data | 850197 |
| padding | 895 |
| unused | 48640 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 1 of 4)[a].adf:bootblock | 1 | 21 | 1 |
| logged disk load from Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 1 of 4)[a].adf boot-block load | 1 | 286 | 9 |
| logged disk load from Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 1 of 4)[a].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 594 | 22 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1599 | 39 |

## Cases handled

- seek & destroy (1993)(mindscape)[cr cls](disk 1 of 4)[a]/Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 1 of 4)[a].adf: custom-format (non-DOS) disk, 1662 non-empty blocks: matched against track-loaded segments
- seek & destroy (1993)(mindscape)[cr cls](disk 1 of 4)[a]/Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 1 of 4)[a].adf: the boot block reads 7680 bytes from disk offset 850944 to an address (found statically); recovered as a unit
- seek & destroy (1993)(mindscape)[cr cls](disk 1 of 4)[a]/Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 1 of 4)[a].adf: the boot block reads 1536 bytes from disk offset 3072 to an address (found statically); recovered as a unit
- runtime segment an address: 367 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 644 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- seek & destroy (1993)(mindscape)[cr cls](disk 1 of 4)[a]/Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 1 of 4)[a].adf: 2 logged loads cover 9216 bytes of the image
- seek & destroy (1993)(mindscape)[cr cls](disk 1 of 4)[a]/Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 1 of 4)[a].adf: blocks 2-2 (bytes 1024-1536) outside the filesystem hold data, not code (opcode density 0.016, entropy 3.55)
- seek & destroy (1993)(mindscape)[cr cls](disk 1 of 4)[a]/Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 1 of 4)[a].adf: blocks 11-1053 (bytes 5632-539648) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.92)
- seek & destroy (1993)(mindscape)[cr cls](disk 1 of 4)[a]/Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 1 of 4)[a].adf: blocks 1055-1059 (bytes 540160-542720) outside the filesystem hold data, not code (opcode density 0.026, entropy 2.93)
- 1 packed track region(s), 305152 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `modify_global_bits` (runtime segment an address): Modifies specific bits in a global variable and writes certain values to memory-mapped registers or RAM before calling another routine. Kind: disk hardware/sync check. Pass path: the branch where bVar1 is false (unaff_D4 != 0).
- `do_nothing_routine` (runtime segment an address): the routine is empty and performs no operations. Kind: loader/sabotage path. Pass path: sub_018C (after call) or sub_0222 (after call).

## Functions


### Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 1 of 4)[a].adf:bootblock

- `issue_boot_io_init_display`: Fills an exec I/O request block with a command, a length and two addresses, issues it through DoIO, then calls a library vector.

### logged disk load from Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 1 of 4)[a].adf boot-block load

- `write_helper_value_to_color01`: Calls a helper twice, storing each returned value both into memory and into the COLOR01 palette register, and advances the memory pointer after each store.
- `adjust_nibbles_combine`: Compares the nibbles of two values and adjusts each nibble toward the target by incrementing or decrementing it, then combines the results.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `clear_memory_array`: Writes zero to each of 8000 longword entries in a loop, starting at a stored address.
- `open_graphics_and_intuition`: Opens graphics.library and intuition.library by calling the exec OpenLibrary vector twice, and stores each returned base in a global.
- `process_text_lines_until_null`: Walks through a text string one character at a time, stopping at a null, and calls one routine for each line of text.
- `count_string_dispatch`: Counts characters in a string until a newline or terminator, then calls several routines when the count is non-zero.
- `initialise_font_pointers`: Stores the same global address into six pointer fields, then calls library vectors at offsets -480 and -72.
- `call_two_library_vectors`: Calls the library vector at offset -78, then calls the one at offset -486 as an indirect call, since the decompiler could not recover a jump table there.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `compare_string_buffers`: Compares a buffer of characters against another buffer, performing case-insensitive checks and potentially calling another routine if a certain character is encountered.
- `sync_global_value`: Checks if a provided value matches a global variable, then increments a global counter while checking a memory-mapped value against a bitmask.
- `set_global_state`: Sets a global variable to a specific value and writes certain values to memory-mapped registers or RAM before calling another routine.
- `execute_recursive_loop`: calls another routine repeatedly until that routine returns zero.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `modify_global_bits`: Modifies specific bits in a global variable and writes certain values to memory-mapped registers or RAM before calling another routine.
- `tiny_moveq`: Small routine of 1 instruction that loads 135 into d1.
- `write_constant_to_memory`: Calls another routine, then writes a constant byte to a memory location pointed to by a register, writes a byte from a register to that same location, and writes the constant again.
- `do_nothing_routine`: the routine is empty and performs no operations.
- `modify_bit_and_wait`: Clears or sets a bit in a memory location and loops until a certain bit in a memory location is or is not set.
- `modify_register_and_loop_2`: Clears a bit in a memory location pointed to by a register, sets a bit in that same location, updates a global variable, and loops until a bit in that global variable is set.
- `modify_register_and_loop`: Modifies a bit in a register, updates a global variable, and loops until a bit in that global variable is set.
- `wait_for_memory_change`: The routine loops indefinitely while a byte at an offset from a base pointer remains equal to itself.
- `update_global_and_loop`: Updates a global variable by clearing or setting a bit and then loops while comparing a value against another value.
- `search_and_copy_data_block`: Searches a memory range for a specific word, and if found, copies a block of data or a sequence of values to a calculated destination address.
- `tiny_move`: Small routine of 1 instruction that copies (a5)+ to d1.
- `calculate_input_mask_sum`: The routine computes a value by masking an input register and a pointer value with a global mask, doubling the pointer result, and adding them together.
- `xor_array_and_mask`: Iterates through an array, XORing its contents into a register, and returns a masked result.
- `copy_string_blocks`: Iterates through a table of pointers and copies a block of bytes from a source address to a destination address, repeating or incrementing a global counter if certain conditions are met.
- `compare_and_increment_global`: compares a register value against a global variable and, if equal, increments a global counter after verifying a bitmask against a memory-mapped register.
- `clear_stored_block_or_cycle`: If a stored pointer is set and the requested size fits, it zeroes that many bytes, advances the pointer and calls a helper.
- `initialize_runtime_tables`: Initializes several global arrays and tables by zeroing memory or filling them with incremental or constant values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_ori_rts`: Small routine of 2 instructions that merges 0 into d2, then returns.
- `update_global_state_and_branch`: Updates a global variable using fixed arithmetic and calls helper routines based on the resulting value, finally storing the value sixteen in a global variable.
- `update_display_state_and_dma_parameters`: Manages a state machine that updates a set of three global values based on a counter and performs conditional resets of memory locations.
- `configure_dma_transfer_2`: Performs a coordinate transformation, swaps global pointers, and sets up hardware registers for a DMA transfer while waiting for a status flag.
- `transform_coordinate_table`: Iterates through a table of coordinates and applies a linear transformation using scaling and rotation factors derived from a data structure.
- `calculate_area_product`: Calculates the product of two differences derived from coordinates stored in a table.
- `update_bounding_box`: Calculates a bounding box based on two input points and updates a structure with the resulting dimensions and pointers.
- `initialize_structure_values`: Sets several constant word values within a specific data structure after waiting for a bit flag in a memory location.
- `reset_memory_blocks`: Sets a series of memory locations to specific constant values.
- `swap_and_update_pointers`: Swaps a longword between two global variables and then updates a series of global pointers and their high-word components using offsets from the new value.
- `update_pointer_table`: Calculates three different offsets from a base value and stores the resulting longwords into a series of global variables.
- `configure_dma_transfer`: Configures a DMA transfer structure by writing a source address, destination address, and certain length or control values, while waiting for a status flag or certain bit to clear.
- `zero_memory_regions`: Clears two separate memory regions by writing zeros to them in a loop.
- `reset_memory_blocks_and_state`: Clears memory regions, updates pointer tables, and resets certain registers or state variables while waiting for a hardware status flag.
- `copy_data_blocks`: Iterates through a source buffer and copies eight-byte chunks from a table to a destination based on values found in the buffer.
- `update_state_and_bounds`: Updates a global variable and checks its value against a range of bounds to assign a new value to a state variable.
- `update_audio_frequency_and_display_state`: Calculates a new audio frequency or channel assignment based on the value of a global variable and updates hardware registers.
- `copy_table_and_reset_registers`: Iterates through a pointer-based table to copy a sequence of words to a fixed memory location, then updates hardware registers including audio or DMA-related or interrupt-related registers and certain memory-mapped or global values.
- `copy_memory_block`: Checks a global variable against two specific constant values.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `process_string_stream`: Parses a source buffer to decode or decompress a stream of bytes, inserting or replacing certain byte values into a destination buffer while handling termination or length-limited sequences.
- `parse_string_with_escapes_2`: Iterates through a source string and writes characters or escaped values to a destination buffer, handling certain byte values as special escape sequences.
- `parse_string_with_escapes`: Parses a source string and writes characters or escaped values to a destination buffer, handling certain byte values as special escape sequences.
- `parse_string_with_escapes_alt`: Iterates through a source string and writes characters or escaped values to a destination buffer, specifically handling certain byte values as escape sequences.
- `write_short_value`: Writes a two-byte value to a memory location, optionally preceded by a specific byte.
- `expand_string_with_padding_2`: Iterates through a source buffer and writes bytes to a destination, expanding certain byte values into a sequence of zero-filled bytes.
- `expand_string_with_padding`: Copies bytes from a source to a destination, expanding certain byte values into a sequence of zero-filled bytes.
- `expand_string_with_padding_alt`: Copies bytes from a source to a destination, expanding certain byte values into a sequence of zero-filled bytes, and stopping if a certain byte is encountered.
- `tiny_move_rts`: Small routine of 2 instructions that copies (a1)+ to (a0)+, then returns.
- `expand_nibble_fields_into_tables`: Copies a table of words and then splits each word's nibbles into three separate longword tables containing scaled values and calculated differences.
- `update_table_with_checksummed_values`: This routine iterates through a table, calculating a checksum-like value or modified pointer or value and storing the result into a destination buffer.
- `update_table_with_sum_and_mask`: This routine iterates through a table of 32-bit values, calculating a sum of certain instruction-like or data-like words and storing a masked, bit-manipulated version of the result into a destination buffer.
- `update_address_table_with_offsets`: This routine updates a table of pointers by adding or copying values.
- `update_table_with_offset_sums`: This routine iterates through a table, calculating the sum of certain instruction or data values and storing the result or a bit-masked version of the sum into a destination buffer.
- `update_table_with_sum_and_mask_2`: This routine iterates through a table, calculating the sum of certain memory or instruction-based values and storing the result or a masked version of the sum into a destination buffer.
- `update_address_table_with_offsets_2`: Updates a table of pointers by adding or copying values, or calculates and stores a bit-masked combination of high-order bits from several pointer-related values into a destination buffer.
- `tiny_ori_2`: Small routine of 1 instruction that merges 1 into d2.
- `update_address_table_with_clamping`: Updates a table of pointers by adding an offset to existing values and clamps the high-order bits of the resulting or modified values to a maximum range.
- `copy_and_clear_word_table`: Copies thirty-two words from a source table into a target table, zeroing each source word as it goes, then stores a fixed address and a zero word into a record and loops until a global becomes zero.
