# Game 1647 (seek & destroy (1993)(mindscape)[cr cls]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01647). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3628816 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 184 raw-word lines |
| Functions named and described | pass, 76 of 76 named (0 author symbols), 76 of 76 described (8 by rule, 53 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1009 of 1009 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2682 of 2682 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 19705 |
| code | 10154 |
| data | 2876756 |
| padding | 793 |
| unused | 721408 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 1 of 4).adf:bootblock | 1 | 34 | 2 |
| logged disk load from Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 1 of 4).adf boot-block load | 1 | 286 | 9 |
| logged disk load from Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 1 of 4).adf boot-block load | 1 | 0 | 0 |
| logged disk load from Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 1 of 4).adf boot-block load | 1 | 169 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 594 | 22 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1599 | 39 |

## Cases handled

- seek & destroy (1993)(mindscape)[cr cls](disk 1 of 4)/Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 1 of 4).adf: custom-format (non-DOS) disk, 1670 non-empty blocks: matched against track-loaded segments
- seek & destroy (1993)(mindscape)[cr cls](disk 2 of 4)/Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 2 of 4).adf: custom-format (non-DOS) disk, 1738 non-empty blocks: matched against track-loaded segments
- seek & destroy (1993)(mindscape)[cr cls](disk 3 of 4)/Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 3 of 4).adf: custom-format (non-DOS) disk, 1700 non-empty blocks: matched against track-loaded segments
- seek & destroy (1993)(mindscape)[cr cls](disk 4 of 4)/Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 4 of 4).adf: custom-format (non-DOS) disk, 514 non-empty blocks: matched against track-loaded segments
- seek & destroy (1993)(mindscape)[cr cls](disk 1 of 4)/Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 1 of 4).adf: the boot block reads 7680 bytes from disk offset 850944 to an address (found statically); recovered as a unit
- seek & destroy (1993)(mindscape)[cr cls](disk 1 of 4)/Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 1 of 4).adf: the boot block reads 1536 bytes from disk offset 3072 to an address (found statically); recovered as a unit
- seek & destroy (1993)(mindscape)[cr cls](disk 1 of 4)/Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 1 of 4).adf: the boot block reads 4096 bytes from disk offset 878592 to an address (found statically); recovered as a unit
- runtime segment an address: 367 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 642 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- seek & destroy (1993)(mindscape)[cr cls](disk 1 of 4)/Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 1 of 4).adf: 3 logged loads cover 13312 bytes of the image
- seek & destroy (1993)(mindscape)[cr cls](disk 1 of 4)/Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 1 of 4).adf: blocks 2-2 (bytes 1024-1536) outside the filesystem hold data, not code (opcode density 0.016, entropy 3.55)
- seek & destroy (1993)(mindscape)[cr cls](disk 1 of 4)/Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 1 of 4).adf: blocks 11-1053 (bytes 5632-539648) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.92)
- seek & destroy (1993)(mindscape)[cr cls](disk 1 of 4)/Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 1 of 4).adf: blocks 1055-1059 (bytes 540160-542720) outside the filesystem hold data, not code (opcode density 0.026, entropy 2.93)
- seek & destroy (1993)(mindscape)[cr cls](disk 2 of 4)/Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 2 of 4).adf: blocks 2-3 (bytes 1024-2048) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.85)
- seek & destroy (1993)(mindscape)[cr cls](disk 2 of 4)/Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 2 of 4).adf: blocks 11-1746 (bytes 5632-894464) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.57)
- seek & destroy (1993)(mindscape)[cr cls](disk 3 of 4)/Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 3 of 4).adf: blocks 2-3 (bytes 1024-2048) outside the filesystem hold data, not code (opcode density 0.006, entropy 2.38)
- seek & destroy (1993)(mindscape)[cr cls](disk 3 of 4)/Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 3 of 4).adf: blocks 11-1708 (bytes 5632-875008) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.52)
- seek & destroy (1993)(mindscape)[cr cls](disk 4 of 4)/Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 4 of 4).adf: blocks 2-2 (bytes 1024-1536) outside the filesystem hold data, not code (opcode density 0.031, entropy 2.97)
- seek & destroy (1993)(mindscape)[cr cls](disk 4 of 4)/Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 4 of 4).adf: blocks 20-531 (bytes 10240-272384) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.65)
- seek & destroy (1993)(mindscape)[cr cls](disk 4 of 4)/Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 4 of 4).adf: blocks 533-533 (bytes 272896-273408) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.53)
- 1 packed track region(s), 305152 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `modify_global_bits` (runtime segment an address): Modifies specific bits in a global variable and writes certain values to memory-mapped registers or RAM before calling another routine. Kind: disk hardware/sync check. Pass path: the branch where bVar1 is false (the code following the check).
- `return_immediately` (runtime segment an address): The routine is empty and performs no operations. Kind: sabotage path. Pass path: the instructions immediately following the call.

## Functions


### Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 1 of 4).adf:bootblock

- `issue_boot_io_init_display`: Fills an exec I/O request block with a command, a length and two addresses, issues it through DoIO, then calls a library vector.
- `set_fields_then_call_library`: Sets three fields of a structure to fixed values, calls the library vector at offset -456, and returns.

### logged disk load from Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 1 of 4).adf boot-block load

- `write_helper_value_to_color01`: Calls a helper twice, storing each returned value both into memory and into the COLOR01 palette register, and advances the memory pointer after each store.
- `adjust_nibbles_combine`: Compares the nibbles of two values and adjusts each nibble toward the target by incrementing or decrementing it, then combines the results.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `clear_memory_array`: Writes zero to each of 8000 longword entries in a loop, starting at a stored address.
- `open_graphics_and_intuition`: Opens graphics.library and intuition.library by calling the exec OpenLibrary vector twice, and stores each returned base in a global.
- `process_text_lines_until_null`: Walks through a text string one character at a time, stopping at a null, and calls one routine for each line of text.
- `count_string_dispatch`: Counts characters in a string until a newline or terminator, then calls several routines when the count is non-zero.
- `initialise_font_pointers`: Stores the same global address into six pointer fields, then calls library vectors at offsets -480 and -72.
- `call_two_library_vectors`: Calls the library vector at offset -78, then calls the one at offset -486 as an indirect call, since the decompiler could not recover a jump table there.

### logged disk load from Seek & Destroy (1993)(Mindscape)[cr CLS](Disk 1 of 4).adf boot-block load

- `shift_bit_stream`: Shifts bits from a register and updates a value using a bitmask or a value from a table.
- `lookup_table_masked_value`: The routine reads a 16-bit value from a table using an index derived from the input word, and returns that value masked with an input mask.
- `fill_two_tables`: The routine calls another routine to obtain a count, then fills a short table with that many values returned by the same routine, accumulating the values.
- `compute_running_difference_filter`: Runs fifteen passes over a 16-word input, writing running sums and differences into an output array at fixed offsets.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `search_and_compare_strings`: Compares a string in memory against a source string by iterating through blocks, performing case-insensitive comparisons, and calling another routine if a match is found.
- `sync_global_value`: Checks if a provided value matches a global variable, then increments a global counter while checking a memory-mapped value against a bitmask.
- `reset_and_reinitialize_state`: Clears a global variable and modifies specific bits within another global variable.
- `execute_recursive_loop`: Calls another routine repeatedly until that routine returns zero.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `modify_global_bits`: Modifies specific bits in a global variable and writes certain values to memory-mapped registers or RAM before calling another routine.
- `tiny_moveq`: Small routine of 1 instruction that loads 135 into d1.
- `write_constant_to_memory`: Calls another routine, then writes a constant byte to a memory location pointed to by a register, writes a byte from a register to the same location, and writes the constant again.
- `return_immediately`: The routine is empty and performs no operations.
- `modify_bit_and_wait_2`: Clears or sets a bit in a memory location and loops until a certain bit in a memory location is or is not set.
- `modify_bit_and_wait`: Clears specific bits in a memory location via a pointer and then enters a loop that polls a global variable until its least significant bit is set.
- `modify_register_and_loop`: Modifies a bit in a register, updates a global variable, and loops until a bit in that global variable is set.
- `wait_for_memory_change`: The routine loops indefinitely while a byte at an offset from a base pointer remains equal to itself, effectively waiting for an external change or acting as an infinite loop if the value is constant.
- `update_global_and_loop`: Updates a global variable by clearing or setting a bit and then loops while comparing a value against another value.
- `search_and_copy_data_block`: Searches a memory range for a specific word, and if found, copies a block of data or a sequence of values to a calculated destination address.
- `tiny_move`: Small routine of 1 instruction that copies (a5)+ to d1.
- `calculate_input_mask_sum`: The routine computes a value by masking an input register and a pointer value with a global mask, doubling the pointer result, and adding them together.
- `xor_array_and_mask`: Iterates through an array, XORing its contents into a register, and returns a masked result.
- `copy_string_blocks`: Iterates through a table of pointers and copies a block of bytes from a source address to a destination address, repeating or incrementing a global counter if certain conditions are met.
- `compare_and_increment_global`: Compares a register value against a global variable and, if equal, increments a global counter after verifying a bitmask against a memory-mapped register.
- `clear_stored_block_or_cycle`: If a stored pointer is set and the requested size fits, it zeroes that many bytes, advances the pointer and calls a helper.
- `initialize_hardware_and_memory_tables`: Detects AGA hardware, extra memory, and 68020 processor capabilities before initializing several large memory blocks.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_ori_rts`: Small routine of 2 instructions that merges 0 into d2, then returns.
- `update_global_state_and_branch`: Updates a global variable using fixed arithmetic and calls helper routines based on the resulting value, finally storing the value sixteen in a global variable.
- `update_display_state_and_dma_parameters`: Manages a state machine that updates a set of three global values based on a counter and performs conditional resets of memory locations.
- `configure_dma_transfer_2`: Performs a coordinate transformation, swaps global pointers, and sets up hardware registers for a DMA transfer while waiting for a status flag.
- `transform_coordinate_table`: Iterates through a table of coordinates and applies a linear transformation using scaling and rotation factors derived from a data structure.
- `calculate_area_product`: Calculates the product of two differences derived from coordinates stored in a table.
- `update_bounding_box`: Calculates a bounding box based on two input points and updates a structure with the resulting dimensions and pointers.
- `initialize_structure_values`: Sets several constant word values within a specific data structure.
- `reset_memory_blocks`: Sets a series of memory locations to specific constant values.
- `swap_global_values`: Swaps the values of two global variables and updates several other global variables with the original value and calculated offsets.
- `update_pointer_table`: Calculates three different offsets from a base value and stores the resulting longwords into a series of global variables.
- `configure_dma_transfer`: Sets up parameters for a DMA transfer by writing to a structure and checking a status flag.
- `zero_memory_regions`: Clears two separate memory regions by writing zeros to them in a loop.
- `reset_memory_blocks_and_state`: Clears memory regions, updates pointer tables, and resets certain registers or state variables while waiting for a hardware status flag.
- `copy_data_blocks`: Iterates through a source buffer and copies eight-byte chunks from a table to a destination based on values found in the buffer.
- `update_state_and_recalculate_index`: Updates a global state variable and performs conditional logic to assign a new value to another global variable based on several threshold comparisons.
- `update_audio_frequency_and_display_state`: Calculates a new audio frequency or channel assignment based on the value of a global variable and updates hardware registers.
- `copy_table_and_reset_registers`: Iterates through a pointer-based table to copy a sequence of words to a fixed memory location, then updates hardware registers including audio or DMA-related or interrupt-related registers and certain memory-mapped or global values.
- `copy_memory_block`: Checks a global variable against two specific constant values.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `run_length_encode_buffer`: Scans an input byte buffer of known length and writes a compressed output into a global destination buffer, collapsing runs of zero bytes into a marker followed by a run count and passing other bytes through, with the 255 byte handled as an escape.
- `parse_encoded_string_2`: Iterates through a source string and copies characters to a destination buffer, handling specific control characters and length constraints.
- `process_string_data`: Parses a source string and copies characters into a destination buffer while handling specific control characters and length constraints.
- `parse_encoded_string`: Iterates through a source buffer to extract characters and handle special markers, writing the results to a destination buffer.
- `write_short_value`: Writes a two-byte value to a memory location, optionally preceded by a specific byte.
- `expand_compressed_bytes_2`: Decodes a byte stream by expanding sequences of bytes, specifically handling markers that trigger zero-fill operations or repeated values.
- `expand_compressed_bytes`: Reads a source buffer and expands sequences of bytes, specifically handling a marker that triggers a zero-fill operation.
- `decompress_byte_stream`: Decodes a stream of bytes where a specific marker indicates a sequence of repeated values or a termination.
- `tiny_move_rts`: Small routine of 2 instructions that copies (a1)+ to (a0)+, then returns.
- `expand_nibble_fields_into_tables`: Copies a table of words and then splits each word's nibbles into three separate longword tables containing scaled values and calculated differences.
- `update_coordinate_table`: Iterates through a table of values, adding offsets to each entry and packing the resulting high-order bits into a new destination table.
- `update_coordinate_table_2`: Iterates through a table of values, adding offsets to each entry and packing the resulting high-order bits into a new destination table.
- `update_coordinate_table_3`: Iterates through a table of values, adding offsets to each entry and packing the resulting high-order bits into a new destination table.
- `update_sprite_attribute_table`: Iterates through a series of memory locations to perform arithmetic on values and pack the resulting high-byte components into a destination table.
- `update_coordinate_table_4`: Iterates through a table of values, adding offsets to each entry and storing the result.
- `update_sprite_attribute_table_2`: Iterates through a table of values, performing arithmetic additions and bitwise manipulations on the data.
- `tiny_ori_2`: Small routine of 1 instruction that merges 1 into d2.
- `update_address_table_with_clamping`: Updates a table of pointers by adding an offset to existing values and clamps the high-order bits of the resulting or modified values to a maximum range.
- `copy_and_clear_word_table`: Copies thirty-two words from a source table into a target table, zeroing each source word as it goes, then stores a fixed address and a zero word into a record and loops until a global becomes zero.
