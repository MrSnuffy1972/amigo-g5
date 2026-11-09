# Game 1573 (no second prize (1990)(thalion)[cr ministry][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01573). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (909424 top-level bytes in 26 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 57 of 57 named (0 author symbols), 57 of 57 described (1 by rule, 45 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1871 of 1871 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2084 of 2084 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 24316 |
| code | 7674 |
| data | 870265 |
| padding | 1 |
| unused | 7168 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| No Second Prize (1990)(Thalion)[cr Ministry][a].adf:bootblock | 1 | 17 | 1 |
| logged disk load from No Second Prize (1990)(Thalion)[cr Ministry][a].adf boot-block load | 1 | 94 | 4 |
| logged disk load from No Second Prize (1990)(Thalion)[cr Ministry][a].adf boot-block load | 1 | 56 | 2 |
| logged disk load from No Second Prize (1990)(Thalion)[cr Ministry][a].adf boot-block load | 1 | 90 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1827 | 45 |

## Cases handled

- no second prize (1990)(thalion)[cr ministry][a]/No Second Prize (1990)(Thalion)[cr Ministry][a].adf: custom-format (non-DOS) disk, 1741 non-empty blocks: matched against track-loaded segments
- extended reference run ignored: it does not re-execute 224 of the 2466 PCs the canonical run logged (2242 PCs logged; the harness no longer reproduces the game), so its PCs and RAM are not used
- no second prize (1990)(thalion)[cr ministry][a]/No Second Prize (1990)(Thalion)[cr Ministry][a].adf: the boot block reads 16896 bytes from disk offset 805376 to an address (found statically); recovered as a unit
- no second prize (1990)(thalion)[cr ministry][a]/No Second Prize (1990)(Thalion)[cr Ministry][a].adf: the boot block reads 2560 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- no second prize (1990)(thalion)[cr ministry][a]/No Second Prize (1990)(Thalion)[cr Ministry][a].adf: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- runtime segment an address (play-rs-m1500-unpk01): 1669 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- no second prize (1990)(thalion)[cr ministry][a]/No Second Prize (1990)(Thalion)[cr Ministry][a].adf: 3 logged loads cover 25088 bytes of the image
- no second prize (1990)(thalion)[cr ministry][a]/No Second Prize (1990)(Thalion)[cr Ministry][a].adf: blocks 22-1564 (bytes 11264-801280) hold 24 packed streams (IMP: File Imploder, TPWM: Turbo Packer) found by scanning, 98% of the bytes
- no second prize (1990)(thalion)[cr ministry][a]/No Second Prize (1990)(Thalion)[cr Ministry][a].adf: blocks 1606-1756 (bytes 822272-899584) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.87)
- no second prize (1990)(thalion)[cr ministry][a]/No Second Prize (1990)(Thalion)[cr Ministry][a].adf: blocks 1758-1758 (bytes 900096-900608) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### No Second Prize (1990)(Thalion)[cr Ministry][a].adf:bootblock

- `run_bit_test_loop`: Boot block entry of seventeen instructions that makes one library request through a stored pointer, then runs a bit-test loop over a value read from memory, calling a helper routine for each group of bits.

### logged disk load from No Second Prize (1990)(Thalion)[cr Ministry][a].adf boot-block load

- `decompress_bitstream_data`: Reads a bitstream to reconstruct a byte array by interpreting bit patterns as either literal bytes or length-prefixed sequences.
- `write_decrementing_global_until_wrap`: Writes a decrementing word to a global variable in a loop that runs until the counter wraps, then returns the value minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from No Second Prize (1990)(Thalion)[cr Ministry][a].adf boot-block load

- `disable_chips_then_idle_loop`: Entry of thirty-five instructions that loops forever if its argument is zero; otherwise it writes masks that clear the top bit, which disable DMA and interrupts, runs a delay countdown, and loops forever writing a fixed value to a global variable.
- `read_capped_entry_field`: Reads a field from a linked structure referenced by a global pointer.

### logged disk load from No Second Prize (1990)(Thalion)[cr Ministry][a].adf boot-block load

- `unpack_and_initialize_hardware`: Calls a routine to unpack a data stream into a buffer, then initializes a series of global variables and a loop that clears memory at a hardware register address.
- `depack_stream_into_buffer`: Depacker loop of about sixty-four instructions that reads packed data through the bit reader routines, decoding each token and writing bytes backward into a buffer from its end.
- `return_without_action`: Routine whose shown body is only a return, so it does no work in the pseudo-code.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_graphics_and_audio_state_2`: Copies a data table into a state structure, modifies hardware registers to set specific bitmasks, and clears eight 16-bit words in a memory range via a loop.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `initialize_graphics_and_audio_state`: Clears several memory tables, initializes a structure with coordinate and transformation data, and performs a series of checks on hardware status registers to synchronize audio and graphics processing.
- `calculate_clipping_rectangle`: Clamps a set of input coordinates against a bounding box, calculates the resulting width and height, and uses these dimensions to compute a memory offset into a table for a sprite or object structure.
- `apply_complex_coordinate_transformations`: Performs four sequential loops that apply a complex 2D rotation and scaling transformation to sets of nine coordinate pairs stored in tables, writing the results to a structure pointed to by a5.
- `process_rectangle_data`: Initializes a structure in a6 with specific values and then performs four separate loops that read index pairs from a table, fetch 16-bit words from a data table using those indices, and call another routine.
- `clamp_and_interpolate_rectangles`: Updates a bounding box in a structure with the minimum and maximum values from four input registers, then performs linear interpolation to clamp a second rectangle's coordinates within the bounds of the first.
- `calculate_structure_offsets`: Performs arithmetic on registers to calculate a value, then stores several 16-bit and 32-bit values into a structure pointed to by a6 based on conditions and bit tests.
- `increment_and_dispatch_sequence`: Increments a 16-bit value at offset 24 of the structure in a5 and performs a series of conditional checks to either increment various 16-bit words at offsets 34, 36, 38, or 40, update a global variable with a constant, or call other routines including decrement_and_search_table, process_data_sequence, decrement_and_verify_table, decrement_data_sequence, bulk_copy_data_blocks, and dispatch_state_handler_sequence.
- `decrement_and_search_table`: Decrements a 16-bit value at offset 30 of a structure in a5 and, if it reaches zero, iterates through a table to find a value matching specific bit patterns.
- `process_data_sequence`: Decrements a 16-bit word at offset 46 of a structure in a5 and, if the result is zero, sets that word to 2 and calls another routine eleven times using various memory addresses in a0.
- `increment_table_pointer`: Checks a value at a pointer in a0 against three different bitmasks and increments the pointer by 256, 16, or 1 respectively if the bits are not set.
- `decrement_and_verify_table`: Decrements a 16-bit value at offset 30 of a structure in a5 and, if zero, checks a table for specific bit patterns and subtracts from a pointer if they are found.
- `decrement_data_sequence`: Decrements a 16-bit word at offset 46 of a structure in a5 and, if the result is zero, sets that word to 2, clears d1, and calls another routine eleven times using various memory addresses in a0.
- `subtract_table_indices`: Checks if a value at a pointer in a0 matches specific bitmasks and subtracts 256, 16, or 1 from that pointer if a match is found, then sets a 16-bit value at offset 32 of a structure in a5 if no matches occurred.
- `bulk_copy_data_blocks`: Uses three loops to copy multiple bytes from a source table to three different destination memory areas based on indices retrieved from a table.
- `dispatch_state_handler_sequence`: Reads a state index from a structure and executes a specific subroutine based on that index, incrementing the index or updating a pointer and counter accordingly.
- `copy_structured_data`: Iterates through a loop to copy a single byte and seven specific offset bytes from a source table to a destination structure pointed to by a0.
- `setup_parameter_block`: If a bit in a status byte is not set, it stores three 32-bit and three 16-bit values into a structure pointed to by a6, derived from a base address in a5.
- `setup_offset_parameter_block`: If a bit in a status byte is not set, it calculates two 32-bit addresses and stores them along with a 32-bit constant and three 16-bit values into a structure in a6.
- `setup_centered_parameter_block`: If a bit in a status byte is not set, it clears a 16-bit value in a6 and stores two 32-bit addresses, a 32-bit constant, and four 16-bit values into a structure in a6.
- `initialize_structure`: If bit 14 of a byte at offset 2 of a structure in a6 is not set, it clears a word at offset 112, stores a pointer derived from a value at offset 62 of a5 plus 3500 into offset 80, sets offset 76 and 84 to that pointer minus 70, writes a long value to offset 64, writes 118 to offsets 98, 100, and 102, and writes 1611 to offset 88.
- `initialize_structure_7000_offset`: If bit 14 of a byte at offset 2 of a structure in a6 is not set, it clears a word at offset 112, stores a pointer derived from a value at offset 62 of a5 plus 7000 into offset 80, sets offset 76 and 84 to that pointer minus 70, writes a long value to offset 64, writes 118 to offsets 98, 100, and 102, and writes 1611 to offset 88.
- `initialize_structure_10500_offset`: If bit 14 of a byte at offset 2 of a structure in a6 is not set, it clears a word at offset 112, stores a pointer derived from a value at offset 62 of a5 plus 10500 into offset 80, sets offset 76 and 84 to that pointer minus 70, writes a long value to offset 64, writes 118 to offsets 98, 100, and 102, and writes 1611 to offset 88.
- `initialize_structure_offset`: If bit 14 of a byte at offset 2 of a structure in a6 is not set, it clears a word at offset 112, stores a pointer derived from a value at offset 62 of a5 into offset 80, sets offset 76 and 84 to that pointer plus 68, writes a long value to offset 64, writes 116 to offsets 98, 100, and 102, and writes 1612 to offset 88.
- `initialize_structure_3500_offset`: If bit 14 of a byte at offset 2 of a structure in a6 is not set, it clears a word at offset 112, stores a pointer derived from a value at offset 62 of a5 plus 3500 into offset 80, sets offset 76 and 84 to that pointer plus 68, writes a long value to offset 64, writes 116 to offsets 98, 100, and 102, and writes 1612 to offset 88.
- `initialize_structure_7000_offset_plus_68`: If bit 14 of a byte at offset 2 of a structure in a6 is not set, it clears a word at offset 112, stores a pointer derived from a value at offset 62 of a5 plus 7000 into offset 80, sets offset 76 and 84 to that pointer plus 68, writes a long value to offset 64, writes 116 to offsets 98, 100, and 102, and writes 1612 to offset 88.
- `initialize_structure_10500_offset_plus_68`: If bit 14 of a byte at offset 2 of a structure in a6 is not set, it clears a word at offset 112, stores a pointer derived from a value at offset 62 of a5 plus 10500 into offset 80, sets offset 76 and 84 to that pointer plus 68, writes a long value to offset 64, writes 116 to offsets 98, 100, and 102, and writes 1612 to offset 88.
- `initialize_structure_offset_210`: If bit 14 of a byte at offset 2 of a structure in a6 is not set, it clears a word at offset 112, stores a pointer derived from a value at offset 62 of a5 into offset 80, sets offset 76 and 84 to that pointer plus 210, writes a long value to offset 64, writes 118 to offsets 98, 100, and 102, and writes 1611 to offset 88.
- `initialize_structure_3500_offset_plus_210`: If bit 14 of a byte at offset 2 of a structure in a6 is not set, it clears a word at offset 112, stores a pointer derived from a value at offset 62 of a5 plus 3500 into offset 80, sets offset 76 and 84 to that pointer plus 210, writes a long value to offset 64, writes 118 to offsets 98, 100, and 102, and writes 1611 to offset 88.
- `initialize_structure_7000_offset_plus_210`: If bit 14 of a byte at offset 2 of a structure in a6 is not set, it clears a word at offset 112, stores a pointer derived from a value at offset 62 of a5 plus 7000 into offset 80, sets offset 76 and 84 to that pointer plus 210, writes a long value to offset 64, writes 118 to offsets 98, 100, and 102, and writes 1611 to offset 88.
- `initialize_structure_data`: Checks bit 14 of a byte at offset 2 of a structure in a6, then if clear, writes a 32-bit constant to offset 64, several 16-bit values to offsets 88, 98, 100, and 102, and stores three pointers at offsets 76, 80, and 84 of the structure in a6.
- `update_structure_pointers`: Checks bit 14 of a byte at offset 2 of a structure in a6, then if clear, writes a 32-bit constant to offset 64, several 16-bit values to offsets 88, 100, and 102, and stores two pointers at offsets 80 and 84 of the structure in a6.
- `set_structure_values`: Checks bit 14 of a byte at offset 2 of a structure in a6, then if clear, writes a 32-bit constant to offset 40, several 16-bit values to offsets 58, 64, and 66, and stores two pointers at offsets 50 and 54 of the structure in a6.
- `copy_table_entries`: Calculates an index by incrementing a value at a memory location, masking it, and shifting it, then uses this index to copy twenty 16-bit values from a table to a destination buffer at a specific offset.
- `enable_audio_interrupt_and_wait`: Writes a value to a hardware register, checks a byte at a specific memory location, and calls another routine before returning from an interrupt context.
- `process_audio_and_registers`: Finds the maximum byte value in a list of 128 bytes, uses that value to calculate an offset for a destination buffer, writes 30 32-bit words to a series of memory locations, and clears several bytes and words in memory and the audio registers.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `update_audio_channels_hardware`: Updates four audio channel registers by calling another routine four times with incrementing offsets, then copies a block of data to the DMA controller and updates a counter in a global variable.
- `check_video_status`: Compares a byte from a hardware register with itself and, if they are not equal, enters a loop that decrements a counter before returning.
- `update_audio_channels`: Compares a byte from a hardware register with itself and, if they are not equal, enters a loop that decrements a counter before returning.
- `update_object_state`: Loads an object structure from a table using a nibble-based index, updates its position and properties from a secondary data table, and modifies hardware DMA registers or global variables based on the object's type and internal state.
- `update_offset_value`: Checks a 16-bit value at offset 34 of a structure in a4; if non-zero, it adds or subtracts a byte from offset 33 to the 16-bit value at offset 22, clamps the result to the value at offset 22, clears offset 34, and stores the final 16-bit value at offset 6 of a5 before returning.
- `calculate_offset_value`: Calculates a value based on a byte from a structure and a multiplication, then either adds or subtracts this from a 16-bit value at offset 16 of the structure and stores the result at offset 6 of a5, finally updating a byte at offset 25 of the structure.
- `update_object_property_offset`: Reads a control nibble from a structure to determine how to modify a 16-bit value at offset 16, such as adding or subtracting a byte from offset 3, or performing complex arithmetic involving a secondary value at offset 22.
