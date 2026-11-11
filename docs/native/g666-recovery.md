# Game 666 (Vroom): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00666). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (911002 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 21 raw-word lines |
| Functions named and described | pass, 58 of 58 named (0 author symbols), 58 of 58 described (2 by rule, 53 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1693 of 1696 (99.82%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2624 of 2628 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 42755 |
| code | 9752 |
| data | 597879 |
| padding | 8 |
| unused | 260608 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Vroom.adf:bootblock | 1 | 23 | 1 |
| logged disk load from Vroom.adf boot-block load | 1 | 214 | 5 |
| logged disk load from Vroom.adf boot-block load | 1 | 35 | 0 |
| logged disk load from Vroom.adf boot-block load | 1 | 163 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1293 | 26 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 505 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 364 | 7 |

## Cases handled

- Vroom/Vroom.adf: custom-format (non-DOS) disk, 1248 non-empty blocks: matched against track-loaded segments
- Vroom/Vroom.adf: the boot block reads 20480 bytes from disk offset 584192 to an address (found statically); recovered as a unit
- Vroom/Vroom.adf: the boot block reads 7680 bytes from disk offset 576512 to an address (found statically); recovered as a unit
- Vroom/Vroom.adf: the boot block reads 16384 bytes from disk offset 856064 to an address (found statically); recovered as a unit
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 676 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- RAM snapshot an address: 3 executed PCs whose RAM was overwritten before any snapshot match its bytes 252784-252848: recovered as a unit run at an address
- RAM snapshot an address: 3 executed PCs whose RAM was overwritten before any snapshot match its bytes 254864-254944: recovered as a unit run at an address
- 818 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 2 first-run image unit(s) in 1 version group(s); 594 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- Vroom/Vroom.adf: 3 logged loads cover 44544 bytes of the image
- Vroom/Vroom.adf: blocks 1584-1671 (bytes 811008-856064) outside the filesystem hold data, not code (opcode density 0.015, entropy 7.09)
- 2 packed track region(s), 549888 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Vroom.adf:bootblock

- `perform_io_operations`: Performs three separate I/O operations by setting up a structure in a1 with specific parameters and calling a library routine, then calls a routine at a fixed address and finally jumps to another fixed address.

### logged disk load from Vroom.adf boot-block load

- `copy_data_block`: Writes two words from a source to offsets -230 and -234 of a structure in a6, sets a byte at a memory location to 129, and then copies 100 longwords from a source to a destination starting at address 256 in a loop.
- `initialize_stack_and_registers`: Sets the stack pointer to a specific value, clears the status register, and performs an indirect jump using a value from a4.
- `reverse_and_copy_data_block`: Calculates a length from a data stream, reverses a block of memory of that length into a temporary buffer, and then performs bitwise shifts on a decoded value.
- `decode_variable_length_integers`: Reads a sequence of variable-length integers from a data stream in a2 and uses the decoded values to perform bitwise shifts and additions to reconstruct a larger integer, which is then used to determine how many bytes to read and write to a destination buffer in a0.
- `parse_variable_length_data_stream`: Reads a sequence of variable-length integers from a data stream in a2, where each value determines the length of the subsequent segment, and uses these values to perform a memory copy from one location to another.

### logged disk load from Vroom.adf boot-block load

- `decompress_bitstream_data`: Copies a block of data from a source to a destination and then processes a bitstream to extract and store bytes into a buffer using a table-driven decompression algorithm.
- `store_long_word_fragment`: Moves a long word from a pointer in a0 to d0, extracts a byte into d7, stores that byte as a word at offset of a structure in a5, sets the carry flag, and performs a logical shift right on d0 before returning.
- `process_data_stream`: Performs a series of conditional checks and loops involving bit shifts and calls to other routines to manipulate data between memory locations and registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `modify_byte_and_test_bit`: Sets bit 1 of a byte at an offset from a4, performs a bit test on d1, and modifies d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_hardware`: Configures hardware registers, manages interrupt and DMA settings, and parses a data string to populate tables and clear memory buffers.
- `initialize_and_expand_data_tables`: Initializes several fields in a structure via a5, populates two tables with interleaved source addresses and data pointers using two loops, and performs a string-based expansion where bytes matching a specific character trigger a copy from one buffer to another.
- `expand_encoded_data`: Iterates through a buffer, finds matching bytes in a table to calculate a destination address, and copies eight blocks of 40 bytes from that table into the destination buffer.
- `copy_block_to_low_memory`: Copies 800 bytes from a fixed source table into low memory starting at a fixed low address, one byte at a time.
- `tick_countdown_and_scale_value`: Decrements a countdown byte while it is non-zero, calling another routine when it reaches zero, then rescales a value by adding its low byte multiplied by sixteen and stores it in a global variable.
- `update_data_tables`: Calculates a difference between two values in a global variable, updates the first value, and then performs a loop to copy data from a table to another location based on a byte value read from a table.
- `process_mouse_input_delta`: Reads the mouse port movement register and updates a global variable by applying a mathematical transformation involving an offset and a division by eight.
- `divide_offset_byte_by_eight`: Adds 126 to a byte from a global variable, wraps the result to eight bits, divides it by eight and stores it back in the same location.
- `update_sprite_animation_state`: Checks the status of a joystick button and a bit in a hardware register to determine an animation frame, then updates a byte in a table and calls another routine to handle sprite rendering.
- `copy_strided_bytes_to_bitmap`: Copies eight bytes spaced forty bytes apart from a source to a destination computed from row and column values.
- `wait_with_countdown_loop`: Runs a short countdown loop from a start value taken from a global, returning when it reaches minus one.
- `clear_bit_in_word`: Performs a bitwise AND on a 16-bit word at an offset of 2 from a5 and returns if the result is non-zero.
- `check_input_and_call_routines`: Checks if a bit is set in a memory location, and if so, writes a constant to a memory location and calls up to four other routines before returning via exception return.
- `process_hunk_data`: Iterates through a data block to find a specific byte value, then uses a calculated offset to clear a long value in a table and writes several bytes to a global structure before setting bit 1 of the CIAA Port A Data Register.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `process_audio_logic`: Increments a long value at a global address, compares a byte from a memory location to another, and calls two different routines based on the result.
- `update_audio_channels_and_process_data`: Calls parse_hunk_and_update_structures four times to update audio channel registers, then calculates a value based on a nibble-indexed table lookup and a global variable, clears a 16-bit value in a global variable, and calls parse_hunk_and_update_structures four more times.
- `parse_hunk_and_update_structures`: Parses a data structure by extracting a nibble-based index to look up values in a table, then updates several fields in the structure in a6 and a5.
- `dispatch_structure_nibble_command`: Extracts a nibble from a byte at offset 2 of a structure and branches to different subroutines based on its value.
- `copy_word_from_structure`: Copies a 16-bit word from offset 16 of a structure in a6 to offset 6 of a structure in a5 and returns.
- `update_structure_from_table`: Searches a table using an index from a structure to find a value that satisfies a comparison, then updates a 16-bit field in the structure, clears a byte at offset 22, and sets a byte at offset 22 to one if the new value is less than a 16-bit value at offset 16.
- `adjust_scaled_value`: Modifies a 16-bit value in a structure by adding or subtracting a byte, then optionally performs a table lookup to update a value in a structure pointed to by a5.
- `update_scaled_value`: Calculates a value by scaling a byte from a table or applying a multiplier based on flags in a structure, adds this result to a 16-bit integer at offset 6 of a second structure, and updates a byte at offset 27 of the first structure using a bitmask.
- `process_input_command_byte`: Reads a command byte from a structure, then branches to various subroutines based on the lower and upper nibbles of that byte.
- `extract_nibble_to_structure`: Extracts the lower nibble from a byte at offset 3 of a structure and stores the result into offset 18 of the same structure before returning.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_audio_registers`: Clears a global variable, calls another routine twice, and performs a bitwise OR on a 16-bit value at an offset relative to a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_fire_button_bit`: Extracts the lowest bit of a register, shifts it left by one, and performs a bitwise OR to update bit 1 of the CIAA Port A Data Register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_global_and_trap`: Stores a value into a global variable and executes a software interrupt.
- `initialize_blitter_and_hardware_state`: Configures the blitter and DMA hardware, updates graphics and audio tables, and manages data relocation and stream processing.
- `configure_interrupt_masks`: Writes two 16-bit values from a table into the INTENA and INTENB registers and returns from an interrupt.
- `update_table_and_increment_counter`: Loads a byte from a table via a pointer, uses that value doubled as an index into another table to write a word to a destination pointed to by a1, writes 336 to offset 2 of that destination, increments a long integer at a global address, and loops back to the start if the byte loaded from the first table is non-zero.
- `configure_blitter_parameters`: Sets several hardware registers, calculates values from a table to update a destination pointer, and enters a loop that calls another routine and updates global pointers.
- `process_hunk_relocation_table`: Iterates through a table of data structures, performing fixed-point multiplication to calculate offset values and updating a structure in a6 with the results.
- `copy_data_loop`: Copies 50 words from a source table to a destination table and another destination table using a decrementing loop, then increments a global variable and returns a value from memory if a comparison with a constant passes.
- `configure_blitter_hardware`: Stores several values from memory into specific locations, including a swapped word, and if a specific bit in a hardware register is not set, configures the blitter with a list pointer, a mask, a color, and a mode.
- `setup_blitter_parameters`: Writes several values from memory to specific locations, including a swapped word, and if a specific bit in a hardware register is not set, sets the blitter list pointer, a mask, a color, and a mode.
- `decrement_global_counter`: Checks a 16-bit value in a global variable; if it is negative, it increments a word at an offset of 8260 in a structure in a5 and decrements the global variable before returning.
- `initialize_graphics_and_audio_tables`: Populates several memory tables with calculated values, including a sequence of scaled values, a series of incrementing offsets, and a set of division-based constants.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_string_data`: Finds the maximum byte value in a sequence, uses it to calculate an offset into a data table, and copies strings into a global buffer.
- `mt_end_2`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `update_audio_initialization_flag`: Increments a byte in a global variable and compares a byte from a local constant to another local constant; if the first byte is less than the second, it branches to another routine, otherwise it clears the global variable.
- `initialize_audio_and_memory`: Sets a global variable to zero, calls another routine four times, and if a specific memory location is non-zero, updates the global variable by calling another routine and performing a bitwise OR with 32768.
- `initialize_audio_channels`: Compares a byte from a hardware register with itself and returns if they are equal; otherwise, it enters a loop that calls another routine four times with different hardware register addresses in a5 and a4, then enters a second loop that runs eight times before returning.
- `update_dma_sprite_attributes`: Updates a sprite or object structure by reading values from a table based on an index, modifies a counter or timer value within the structure, and enables DMA by writing to the DMA controller register.
- `process_joystick_input_and_update_state`: Reads the CIAA Port A Data register to check joystick and fire button status, then updates a structure by performing bitwise operations, table lookups, or arithmetic on various offsets based on the input value.
